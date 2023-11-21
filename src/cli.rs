use std::env;

use clap::Parser;
use color_eyre::eyre;
use const_format::concatcp;

use near_account_id::AccountId;

mod call;
mod dissect;

use super::macros::error;

pub use call::CallAction;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    \x1b[1mnearx\x1b[0m call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    [as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] [display]] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    call::EXAMPLES
))]
enum RawCommand {
    Call(call::CallCommand),
    Dissect(dissect::DissectCommand),
}

#[derive(Debug)]
pub enum Command {
    Call(CallCommand),
    Dissect(DissectCommand),
}

impl Command {
    pub fn parse() -> Result<Option<Self>, eyre::Error> {
        match RawCommand::parse() {
            RawCommand::Call(call) => Ok(CallCommand::parse(call)?.map(Command::Call)),
            RawCommand::Dissect(call) => Ok(Some(Command::Dissect(DissectCommand::parse(call)?))),
        }
    }
}

#[derive(Debug)]
pub struct CallCommand {
    pub method: String,
    pub args: serde_json::Value,
    pub contract: AccountId,
    pub account: Option<call::AccountForTx>,
    pub rpc_url: String,
    pub rpc_api_key: Option<near_jsonrpc_client::auth::ApiKey>,
}

impl CallCommand {
    fn parse(call: call::CallCommand) -> Result<Option<Self>, eyre::Error> {
        let mut command = call::ConsumableCommand::default();

        call.apply(&mut command);

        let method = command.method.expect("method should've been set by now");

        let args = command.args;

        let contract = command
            .contract_id
            .expect("contract should've been set by now");

        let account = command.signer_id.map(|signer_id| call::AccountForTx {
            id: signer_id,
            secret_key: command
                .secret_key
                .expect("secret key should've been set by now"),
            deposit: command.deposit,
            gas: command.gas,
            action: command.action,
        });

        let rpc_url =
            if let Some(rpc_url) = command.rpc_url.or_else(|| env::var("NEAR_RPC_URL").ok()) {
                rpc_url
            } else {
                error!(
                    "missing RPC URL, please specify `\x1b[1mthrough \x1b[3m<RPC_URL>\x1b[0m` \
            or set `\x1b[1;3mNEAR_RPC_URL\x1b[0m` environment variable"
                );
                return Ok(None);
            };

        let mut rpc_api_key = command.rpc_api_key;
        if let Ok(api_key) = env::var("NEAR_RPC_API_KEY") {
            rpc_api_key = Some(near_jsonrpc_client::auth::ApiKey::new(api_key)?);
        }

        Ok(Some(CallCommand {
            method,
            args,
            contract,
            account,
            rpc_url,
            rpc_api_key,
        }))
    }
}

#[derive(Debug)]
pub struct DissectCommand {
    pub signed_tx: Option<dissect::SignedTransaction>,
    pub json: bool,
}

impl DissectCommand {
    fn parse(dissect: dissect::DissectCommand) -> Result<Self, eyre::Error> {
        Ok(DissectCommand {
            signed_tx: dissect.signed_tx,
            json: dissect.json,
        })
    }
}
