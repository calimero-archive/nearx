use std::env;

use clap::{Parser, Subcommand};
use color_eyre::eyre;
use const_format::concatcp;

use near_account_id::AccountId;

mod consumers;

use super::macros::error;

const EXAMPLES: &str = "\n
\x1b[1;4mExamples:\x1b[0m
  # Immutably call `\x1b[1madd(1, 2)\x1b[0m` on `\x1b[1madder.testnet\x1b[0m`
  $ nearx call \x1b[1;3m\"add\"\x1b[0m with \x1b[1;3m\"[1, 2]\"\x1b[0m on \x1b[1;3m\"adder.testnet\"\x1b[0m through \x1b[1;3m\"https://rpc.testnet.near.org\"\x1b[0m

  # Mutably call `\x1b[1maddGreeting(\"Hello, World!\")\x1b[0m` on `\x1b[1mgreeter.testnet\x1b[0m` as `\x1b[1mbob.testnet\x1b[0m` with `\x1b[1m300TGas\x1b[0m` and no deposit.
  $ nearx call \x1b[1;3m\"addGreeting\"\x1b[0m with \x1b[1;3m'[\"Hello World\"]'\x1b[0m on \x1b[1;3m\"greeter.testnet\"\x1b[0m as \x1b[1;3m\"bob.testnet\"\x1b[0m with \x1b[1;3m\"ed25519:52CwWhWHzgaSZRx..bMFSyXn9hao4YNXuz\"\x1b[0m through \x1b[1;3m\"https://rpc.testnet.near.org\"\x1b[0m
  
  # Authenticated, immutable call to `\x1b[1madd(1, 2)\x1b[0m` on `\x1b[1madder.testnet\x1b[0m` as `\x1b[1mcarol.testnet\x1b[0m`
  $ nearx call \x1b[1;3m\"add\"\x1b[0m with \x1b[1;3m\"[1, 2]\"\x1b[0m on \x1b[1;3m\"adder.testnet\"\x1b[0m through \x1b[1;3m\"https://rpc.testnet.near.org\"\x1b[0m with \x1b[1;3m\"5a28cd2041c1780f5d64fa6dca4b22bd\"\x1b[0m
  
  # Read `\x1b[1mNEAR_RPC_URL\x1b[0m` and `\x1b[1mNEAR_RPC_API_KEY\x1b[0m` from the environment
  $ nearx call \x1b[1;3m\"add\"\x1b[0m with \x1b[1;3m\"[1, 2]\"\x1b[0m on \x1b[1;3m\"adder.testnet\"\x1b[0m

  # Display a transaction that calls `\x1b[1maddGreeting(\"Hello, World!\")\x1b[0m` on `\x1b[1mgreeter.testnet\x1b[0m` as `\x1b[1mderek.testnet\x1b[0m` with `\x1b[1m100 TGas\x1b[0m` and `\x1b[1m5 Ⓝ\x1b[0m` deposit.
  $ nearx call \x1b[1;3m\"addGreeting\"\x1b[0m with \x1b[1;3m'[\"Hello World\"]'\x1b[0m on \x1b[1;3m\"greeter.testnet\"\x1b[0m as \x1b[1;3m\"derek.testnet\"\x1b[0m with \x1b[1;3m\"ed25519:52CwWhWHzgaSZRx..bMFSyXn9hao4YNXuz\"\x1b[0m gas \x1b[1;3m\"100Tgas\"\x1b[0m deposit \x1b[1;3m\"5N\"\x1b[0m display
";

#[derive(Debug, Parser)]
#[clap(author, about, version)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    \x1b[1mnearx\x1b[0m call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    [as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] [display]] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
pub enum RootCommand {
    Call(CallCommand),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx \x1b[1mcall \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    [as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] [display]] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Calls a method on a contract.
pub struct CallCommand {
    /// Defines the method to call.
    method: String,

    #[clap(subcommand)]
    rest: CallCommandRest,
}

#[derive(Debug, Subcommand)]
enum CallCommandRest {
    With(CallCommandRestWith),
    On(CallCommandRestOn),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m \x1b[1mwith \x1b[3m'{}'\x1b[0m \
    on \x1b[3mCONTRACT\x1b[0m \
    [as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] [display]] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Defines the JSON arguments to pass to the method.
struct CallCommandRestWith {
    /// The JSON arguments to pass to the method.
    #[clap(value_name = "JSON_ARGS", value_parser = serde_json_from_str)]
    args: serde_json::Value,

    #[clap(subcommand)]
    rest: CallCommandRestWithRest,
}

fn serde_json_from_str(s: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(s)
}

#[derive(Debug, Subcommand)]
enum CallCommandRestWithRest {
    On(CallCommandRestOn),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    \x1b[1mon \x1b[3mCONTRACT\x1b[0m \
    [as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] [display]] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Defines the contract to call.
struct CallCommandRestOn {
    /// The contract to call.
    contract: AccountId,

    #[clap(subcommand)]
    rest: Option<CallCommandRestOnRest>,
}

#[derive(Debug, Subcommand)]
enum CallCommandRestOnRest {
    As(CallCommandRestOnRestAs),
    Through(CallCommandThrough),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    \x1b[1mas \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] [display] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Defines the account to call the contract as. (Makes this a signed TX.)
struct CallCommandRestOnRestAs {
    /// The account to call the contract as.
    account: AccountId,

    #[clap(subcommand)]
    rest: CallCommandRestOnRestAsRest,
}

#[derive(Debug, Subcommand)]
enum CallCommandRestOnRestAsRest {
    With(CallCommandRestOnRestAsRestWith),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    as \x1b[3mACCOUNT\x1b[0m \x1b[1mwith \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] [display] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Defines the secret key to use.
struct CallCommandRestOnRestAsRestWith {
    /// The private key to use.
    /// Format: `<ed25519|secp256k1>:<bs58 private key>`
    #[clap(verbatim_doc_comment)]
    private_key: near_crypto::SecretKey,

    #[clap(subcommand)]
    rest: Option<CallCommandRestOnRestAsRestWithRest>,
}

#[derive(Debug, Subcommand)]
enum CallCommandRestOnRestAsRestWithRest {
    Gas(CallCommandRestOnRestAsRestWithRestGas),
    Deposit(CallCommandRestOnRestAsRestWithRestDeposit),
    Display(CallCommandDisplay),
    Through(CallCommandThrough),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m \x1b[1mgas \x1b[3mGAS\x1b[0m [deposit \x1b[3mDEPOSIT\x1b[0m] [display] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Defines the amount of gas to use.
struct CallCommandRestOnRestAsRestWithRestGas {
    /// The amount of gas to use.
    gas: near_primitives::types::Gas,

    #[clap(subcommand)]
    rest: Option<CallCommandRestOnRestAsRestWithRestGasRest>,
}

#[derive(Debug, Subcommand)]
enum CallCommandRestOnRestAsRestWithRestGasRest {
    Deposit(CallCommandRestOnRestAsRestWithRestGasRestDeposit),
    Display(CallCommandDisplay),
    Through(CallCommandThrough),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] \x1b[1mdeposit \x1b[3mDEPOSIT\x1b[0m [display] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Defines the amount of NEAR to deposit.
struct CallCommandRestOnRestAsRestWithRestGasRestDeposit {
    /// The amount of NEAR to deposit.
    deposit: near_primitives::types::Balance,

    #[clap(subcommand)]
    rest: Option<CallCommandRestOnRestAsRestWithRestGasRestDepositRest>,
}

#[derive(Debug, Subcommand)]
enum CallCommandRestOnRestAsRestWithRestGasRestDepositRest {
    Display(CallCommandDisplay),
    Through(CallCommandThrough),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m \x1b[1mdeposit \x1b[3mDEPOSIT\x1b[0m [gas \x1b[3mGAS\x1b[0m] [display] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Defines the amount of NEAR to deposit.
struct CallCommandRestOnRestAsRestWithRestDeposit {
    /// The amount of NEAR to deposit.
    deposit: near_primitives::types::Balance,

    #[clap(subcommand)]
    rest: Option<CallCommandRestOnRestAsRestWithRestDepositRest>,
}

#[derive(Debug, Subcommand)]
enum CallCommandRestOnRestAsRestWithRestDepositRest {
    Gas(CallCommandRestOnRestAsRestWithRestDepositRestGas),
    Display(CallCommandDisplay),
    Through(CallCommandThrough),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [deposit \x1b[3mDEPOSIT\x1b[0m] \x1b[1mgas \x1b[3mGAS\x1b[0m [display] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Defines the amount of gas to use.
struct CallCommandRestOnRestAsRestWithRestDepositRestGas {
    /// The amount of gas to use.
    gas: near_primitives::types::Gas,

    #[clap(subcommand)]
    rest: Option<CallCommandRestOnRestAsRestWithRestDepositRestGasRest>,
}

#[derive(Debug, Subcommand)]
enum CallCommandRestOnRestAsRestWithRestDepositRestGasRest {
    Display(CallCommandDisplay),
    Through(CallCommandThrough),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    [as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] \x1b[1mdisplay\x1b[0m] \
    through \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Displays the transaction that would be sent.
struct CallCommandDisplay {
    #[clap(subcommand)]
    rest: Option<CallCommandDisplayRest>,
}

#[derive(Debug, Subcommand)]
enum CallCommandDisplayRest {
    Through(CallCommandThrough),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    [as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] [display]] \
    \x1b[1mthrough \x1b[3mRPC_URL\x1b[0m [with \x1b[3mTOKEN\x1b[0m]",
    EXAMPLES
))]
/// Defines the RPC URL to connect to.
struct CallCommandThrough {
    /// The RPC URL to connect to. [env: NEAR_RPC_URL]
    #[clap(value_name = "URL")]
    rpc_url: String,

    #[clap(subcommand)]
    rest: Option<CallCommandThroughRest>,
}

#[derive(Debug, Subcommand)]
enum CallCommandThroughRest {
    With(CallCommandThroughRestWith),
}

#[derive(Debug, Parser)]
#[clap(after_help = concatcp!(
    "\x1b[1;4mHint:\x1b[0m\n  \
    nearx call \
    \x1b[3mMETHOD\x1b[0m [with \x1b[3m'{}'\x1b[0m] \
    on \x1b[3mCONTRACT\x1b[0m \
    [as \x1b[3mACCOUNT\x1b[0m with \x1b[3mSECRET\x1b[0m [gas \x1b[3mGAS\x1b[0m] [deposit \x1b[3mDEPOSIT\x1b[0m] [display]] \
    through \x1b[3mRPC_URL\x1b[0m \x1b[1mwith \x1b[3mTOKEN\x1b[0m",
    EXAMPLES
))]
/// Defines the RPC API key to use.
struct CallCommandThroughRestWith {
    /// The RPC API key to use. [env: NEAR_RPC_API_KEY]
    #[clap(value_name = "KEY", value_parser = api_key_from_str)]
    rpc_api_key: near_jsonrpc_client::auth::ApiKey,
}

fn api_key_from_str(s: &str) -> Result<near_jsonrpc_client::auth::ApiKey, String> {
    near_jsonrpc_client::auth::ApiKey::new(s).map_err(|e| e.to_string())
}

#[derive(Debug)]
pub struct Command {
    pub method: String,
    pub args: serde_json::Value,
    pub contract: AccountId,
    pub account: Option<AccountForTx>,
    pub rpc_url: String,
    pub rpc_api_key: Option<near_jsonrpc_client::auth::ApiKey>,
}

#[derive(Debug)]
pub struct AccountForTx {
    pub id: AccountId,
    pub secret_key: near_crypto::SecretKey,
    pub deposit: near_primitives::types::Balance,
    pub gas: near_primitives::types::Gas,
    pub action: super::CallAction,
}

impl TryFrom<RootCommand> for Option<Command> {
    type Error = eyre::Error;

    fn try_from(raw: RootCommand) -> Result<Self, Self::Error> {
        let mut command = consumers::Command::default();

        let RootCommand::Call(call) = raw;

        call.apply(&mut command);

        let method = command.method.expect("method should've been set by now");

        let args = command.args;

        let contract = command
            .contract_id
            .expect("contract should've been set by now");

        let account = command.signer_id.map(|signer_id| AccountForTx {
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

        Ok(Some(Command {
            method,
            args,
            contract,
            account,
            rpc_url,
            rpc_api_key,
        }))
    }
}
