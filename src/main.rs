use color_eyre::eyre;

mod cli;
mod commands;
pub mod macros;
mod utils;

async fn init() -> eyre::Result<()> {
    let command = match cli::Command::parse()? {
        Some(command) => command,
        None => return Ok(()),
    };

    match command {
        cli::Command::Call(command) => commands::call(command).await?,
        cli::Command::Dissect(command) => commands::dissect(command).await?,
    }

    Ok(())
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    init().await?;

    Ok(())
}
