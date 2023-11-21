use borsh::BorshDeserialize;
use color_eyre::eyre;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

use near_primitives::transaction::SignedTransaction;

use crate::cli;
use crate::macros::error;

pub async fn run(command: cli::DissectCommand) -> eyre::Result<()> {
    let signed_tx = match command.signed_tx {
        Some(signed_tx) => signed_tx,
        None => read_signed_tx().await?,
    };

    if command.json {
        println!("{}", serde_json::to_string_pretty(&signed_tx)?);
    } else {
        println!("{:#?}", signed_tx);
    }

    Ok(())
}

pub async fn read_signed_tx() -> eyre::Result<SignedTransaction> {
    let mut stdin = LinesStream::new(BufReader::new(tokio::io::stdin()).lines()).take(5);

    while let Some(line) = stdin.next().await.transpose()? {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let maybe_signed_tx = (|| {
            Ok::<_, eyre::Error>(SignedTransaction::try_from_slice(
                &near_primitives::serialize::from_base64(line)?,
            )?)
        })();

        let signed_tx = match maybe_signed_tx {
            Ok(signed_tx) => signed_tx,
            Err(err) => {
                error!("failed to parse signed transaction: {}", err);
                continue;
            }
        };

        return Ok(signed_tx);
    }

    Err(eyre::eyre!("failed to read signed transaction"))
}
