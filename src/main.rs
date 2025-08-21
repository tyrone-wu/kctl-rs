use cmd::{options, Options};

mod cmd;
mod subcommand;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Options {
        verbose,
        config,
        request,
    } = options().run();
    if verbose {
        tracing_subscriber::fmt::init();
    }
    request.send(verbose, &config).await?;
    Ok(())
}
