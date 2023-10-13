use serde_json::json;

use near_account_id::AccountId;

use super::*;
use crate::{utils, CallAction};

pub struct Command {
    pub method: Option<String>,
    pub args: serde_json::Value,
    pub contract_id: Option<AccountId>,
    pub signer_id: Option<AccountId>,
    pub secret_key: Option<near_crypto::SecretKey>,
    pub gas: near_primitives::types::Gas,
    pub deposit: near_primitives::types::Balance,
    pub action: CallAction,
    pub rpc_url: Option<String>,
    pub rpc_api_key: Option<near_jsonrpc_client::auth::ApiKey>,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            method: None,
            args: json!({}),
            contract_id: None,
            signer_id: None,
            secret_key: None,
            gas: utils::TGAS * 300,
            deposit: utils::NEAR * 0,
            action: CallAction::Submit,
            rpc_url: None,
            rpc_api_key: None,
        }
    }
}

impl CallCommand {
    pub fn apply(self, command: &mut Command) {
        command.method = Some(self.method);
        match self.rest {
            CallCommandRest::With(with_args) => with_args.apply(command),
            CallCommandRest::On(on_contract) => on_contract.apply(command),
        }
    }
}

impl CallCommandRestWith {
    fn apply(self, command: &mut Command) {
        command.args = self.args;
        let CallCommandRestWithRest::On(on_contract) = self.rest;
        on_contract.apply(command);
    }
}

impl CallCommandRestOn {
    fn apply(self, command: &mut Command) {
        command.contract_id = Some(self.contract);
        match self.rest {
            Some(CallCommandRestOnRest::As(with_signer)) => with_signer.apply(command),
            Some(CallCommandRestOnRest::Through(through_rpc)) => through_rpc.apply(command),
            None => {}
        }
    }
}

impl CallCommandRestOnRestAs {
    fn apply(self, command: &mut Command) {
        command.signer_id = Some(self.account);
        let CallCommandRestOnRestAsRest::With(with_secret_key) = self.rest;
        with_secret_key.apply(command);
    }
}

impl CallCommandRestOnRestAsRestWith {
    fn apply(self, command: &mut Command) {
        command.secret_key = Some(self.private_key);
        match self.rest {
            Some(CallCommandRestOnRestAsRestWithRest::Gas(gas)) => gas.apply(command),
            Some(CallCommandRestOnRestAsRestWithRest::Deposit(deposit)) => deposit.apply(command),
            Some(CallCommandRestOnRestAsRestWithRest::Display(display)) => display.apply(command),
            Some(CallCommandRestOnRestAsRestWithRest::Through(through_rpc)) => {
                through_rpc.apply(command)
            }
            None => {}
        }
    }
}

impl CallCommandRestOnRestAsRestWithRestGas {
    fn apply(self, command: &mut Command) {
        command.gas = self.gas;
        match self.rest {
            Some(CallCommandRestOnRestAsRestWithRestGasRest::Deposit(deposit)) => {
                deposit.apply(command)
            }
            Some(CallCommandRestOnRestAsRestWithRestGasRest::Display(display)) => {
                display.apply(command)
            }
            Some(CallCommandRestOnRestAsRestWithRestGasRest::Through(through_rpc)) => {
                through_rpc.apply(command)
            }
            None => {}
        }
    }
}

impl CallCommandRestOnRestAsRestWithRestGasRestDeposit {
    fn apply(self, command: &mut Command) {
        command.deposit = self.deposit;
        match self.rest {
            Some(CallCommandRestOnRestAsRestWithRestGasRestDepositRest::Display(display)) => {
                display.apply(command)
            }
            Some(CallCommandRestOnRestAsRestWithRestGasRestDepositRest::Through(through_rpc)) => {
                through_rpc.apply(command)
            }
            None => {}
        }
    }
}

impl CallCommandRestOnRestAsRestWithRestDeposit {
    fn apply(self, command: &mut Command) {
        command.deposit = self.deposit;
        match self.rest {
            Some(CallCommandRestOnRestAsRestWithRestDepositRest::Gas(gas)) => gas.apply(command),
            Some(CallCommandRestOnRestAsRestWithRestDepositRest::Display(display)) => {
                display.apply(command)
            }
            Some(CallCommandRestOnRestAsRestWithRestDepositRest::Through(through_rpc)) => {
                through_rpc.apply(command)
            }
            None => {}
        }
    }
}

impl CallCommandRestOnRestAsRestWithRestDepositRestGas {
    fn apply(self, command: &mut Command) {
        command.gas = self.gas;
        match self.rest {
            Some(CallCommandRestOnRestAsRestWithRestDepositRestGasRest::Display(display)) => {
                display.apply(command)
            }
            Some(CallCommandRestOnRestAsRestWithRestDepositRestGasRest::Through(through_rpc)) => {
                through_rpc.apply(command)
            }
            None => {}
        }
    }
}

impl CallCommandDisplay {
    fn apply(self, command: &mut Command) {
        command.action = CallAction::Display;
        match self.rest {
            Some(CallCommandDisplayRest::Through(through_rpc)) => through_rpc.apply(command),
            None => {}
        }
    }
}

impl CallCommandThrough {
    fn apply(self, command: &mut Command) {
        command.rpc_url = Some(self.rpc_url);
        match self.rest {
            Some(CallCommandThroughRest::With(with_api_key)) => with_api_key.apply(command),
            None => {}
        }
    }
}

impl CallCommandThroughRestWith {
    fn apply(self, command: &mut Command) {
        command.rpc_api_key = Some(self.rpc_api_key);
    }
}
