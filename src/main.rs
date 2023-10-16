use std::borrow::Cow;

use borsh::BorshSerialize;
use clap::Parser;
use color_eyre::eyre;

use near_account_id::AccountId;
use near_crypto::InMemorySigner;
use near_jsonrpc_client::{methods, JsonRpcClient};

mod cli;
pub mod macros;
mod utils;

use macros::{error, info, log, warn};

async fn init() -> eyre::Result<()> {
    let command = match Option::<cli::Command>::try_from(cli::RootCommand::parse())? {
        Some(command) => command,
        None => return Ok(()),
    };

    let mut client = JsonRpcClient::connect(command.rpc_url);
    if let Some(key) = command.rpc_api_key {
        client = client.header(key);
    }

    if let Some(account) = command.account {
        let signer = near_crypto::InMemorySigner::from_secret_key(account.id, account.secret_key);

        transact(
            client,
            signer,
            command.contract,
            command.method,
            command.args,
            account.gas,
            account.deposit,
            account.action,
        )
        .await?;
    } else {
        view(client, command.method, command.args, command.contract).await?;
    }

    Ok(())
}

async fn view(
    client: JsonRpcClient,
    method: String,
    args: serde_json::Value,
    contract: AccountId,
) -> eyre::Result<()> {
    let request = methods::query::RpcQueryRequest {
        block_reference: near_primitives::types::BlockReference::latest(),
        request: near_primitives::views::QueryRequest::CallFunction {
            account_id: contract,
            method_name: method,
            args: serde_json::to_vec(&args)?.into(),
        },
    };

    let result = match client.call(request).await?.kind {
        near_jsonrpc_primitives::types::query::QueryResponseKind::CallResult(result) => result,
        err => unreachable!("unexpected response kind: {:?}", err),
    };

    for (idx, log) in result.logs.iter().enumerate() {
        log!(
            "#{:>count$}\x1b[0m │ {}",
            idx + 1,
            log,
            count = result.logs.len().to_string().len()
        );
    }

    utils::print_result(Cow::from(result.result));

    Ok(())
}

#[derive(Debug)]
pub enum CallAction {
    Display,
    Submit,
}

async fn transact(
    client: JsonRpcClient,
    signer: InMemorySigner,
    contract: AccountId,
    method: String,
    args: serde_json::Value,
    gas: near_primitives::types::Gas,
    deposit: near_primitives::types::Balance,
    action: CallAction,
) -> eyre::Result<()> {
    let access_key_request = methods::query::RpcQueryRequest {
        block_reference: near_primitives::types::BlockReference::latest(),
        request: near_primitives::views::QueryRequest::ViewAccessKey {
            account_id: signer.account_id.clone(),
            public_key: signer.public_key.clone(),
        },
    };

    let methods::query::RpcQueryResponse {
        block_hash,
        kind: query_response_kind,
        ..
    } = client.call(access_key_request).await?;

    let near_primitives::views::AccessKeyView { permission, nonce } = match query_response_kind {
        near_jsonrpc_primitives::types::query::QueryResponseKind::AccessKey(result) => result,
        err => unreachable!("unexpected response kind: {:?}", err),
    };

    match permission {
        near_primitives::views::AccessKeyPermissionView::FullAccess { .. } => {}
        near_primitives::views::AccessKeyPermissionView::FunctionCall {
            receiver_id,
            method_names,
            ..
        } => {
            if receiver_id != contract.as_str() {
                warn!("access key does not have permission to call this contract");
                return Ok(());
            }

            if !(method_names.is_empty() || method_names.contains(&method)) {
                warn!("access key does not have permission to call this method");
                return Ok(());
            }
        }
    }

    let transaction = near_primitives::transaction::Transaction {
        signer_id: signer.account_id.clone(),
        public_key: signer.public_key.clone(),
        nonce: nonce + 1,
        block_hash,
        receiver_id: contract,
        actions: vec![near_primitives::transaction::Action::FunctionCall(
            near_primitives::transaction::FunctionCallAction {
                method_name: method.clone(),
                args: serde_json::to_vec(&args)?,
                gas,
                deposit,
            },
        )],
    };

    let signed_transaction = transaction.sign(&signer);

    info!(
        "transaction hash: \x1b[1m{}\x1b[0m",
        signed_transaction.get_hash()
    );

    if let CallAction::Display = action {
        println!(
            "{}",
            near_primitives::serialize::base64_display(&signed_transaction.try_to_vec()?)
        );
        return Ok(());
    }

    let request = methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest { signed_transaction };

    let response = client.call(request).await?;

    info!(
        "      block hash: \x1b[1m{}\x1b[0m",
        response.transaction_outcome.block_hash
    );

    let outcome = response.transaction_outcome.outcome;

    info!(
        "  execution cost: \x1b[1m{:.4} TGas\x1b[0m",
        outcome.gas_burnt as f64 / utils::TGAS as f64
    );

    for (idx, log) in outcome.logs.iter().enumerate() {
        log!(
            "#{:>count$}\x1b[0m │ {}",
            idx + 1,
            log,
            count = outcome.logs.len().to_string().len()
        );
    }

    match response.status {
        near_primitives::views::FinalExecutionStatus::SuccessValue(result) => {
            utils::print_result(Cow::from(result));
        }
        near_primitives::views::FinalExecutionStatus::Failure(error) => {
            error!("transaction failed: {:#?}", error);
        }
        _ => unreachable!("unexpected response status: {:#?}", response.status),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    init().await?;

    Ok(())
}
