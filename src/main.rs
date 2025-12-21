mod app;
mod audio;
mod config;
mod download;
mod prompt;
mod ui;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    let mut app = App::new()?;

    // Run the app
    let result = app.run();

    // Print any errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
