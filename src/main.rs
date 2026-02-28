mod app;
mod game;
mod save;
mod ui;

use clap::Parser;

#[derive(Parser)]
#[command(name = "tamimon", version, about = "Terminal Monster - CLI育成放置ゲーム")]
struct Cli {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();
    app::run().await
}
