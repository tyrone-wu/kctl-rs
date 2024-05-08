use cmd::{options, Options};

mod cmd;
mod subcommand;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Options {
        verbose: _verbose,
        config,
        request,
    } = options().run();
    let _ = request.send(&config).await;
    Ok(())
}
