use borsh::BorshDeserialize;
use clap::Parser;
use color_eyre::eyre;

pub use near_primitives::transaction::SignedTransaction;

#[derive(Debug, Parser)]
#[clap(after_help = "\x1b[1;4mHint:\x1b[0m\n  \
    nearx \x1b[1mdissect\x1b[0m [\x1b[1;3mSIGNED_TX\x1b[0m] [--json]\n\
    \n\
    \x1b[1;4mExamples:\x1b[0m
  # Read from positional argument
  $ nearx dissect \x1b[1;3m\"DAAAAG1pcmFj..RiFN4/m1WxBA==\"\x1b[0m
  
  # Read from stdin
  $ nearx dissect < \x1b[1;3m\"DAAAAG1pcmFj..RiFN4/m1WxBA==\"\x1b[0m")]
/// Dissasemble a signed transaction
pub struct DissectCommand {
    /// A base-64 encoded, borsh-compacted, NEAR signed transaction
    #[clap(value_name = "SIGNED_TX", value_parser = signed_tx_from_str)]
    pub signed_tx: Option<SignedTransaction>,

    /// Display the transaction as JSON
    #[clap(long)]
    pub json: bool,
}

fn signed_tx_from_str(s: &str) -> eyre::Result<SignedTransaction> {
    Ok(SignedTransaction::try_from_slice(
        &near_primitives::serialize::from_base64(s)?,
    )?)
}
