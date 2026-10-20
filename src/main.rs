mod app;
mod audio;
mod config;
mod download;
mod prompt;
mod ui;

use anyhow::Result;
use app::App;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {}

fn main() -> Result<()> {
    let _cli = Cli::parse();
    let mut app = App::new()?;

    // Run the app
    let result = app.run();

    // Print any errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
