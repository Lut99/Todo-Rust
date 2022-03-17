/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   16 Mar 2022, 18:01:21
 * Last edited:
 *   17 Mar 2022, 18:29:03
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint for the client-side binary of the Todo tool.
**/

use std::fs::File;

use log::info;
use simplelog::{LevelFilter, WriteLogger};
use todo_client::cli::{Config, ConfigSubcommand};
use todo_client::tui::TerminalUi;


/***** ENTRYPOINT *****/
fn main() {
    // Parse the arguments
    let config = match Config::load() {
        Ok(config) => config,
        Err(err)   => { eprintln!("Could not load configuration: {}", err); std::process::exit(1); }
    };

    // Setup the logger
    let handle = match File::create(&config.log_path.unwrap()) {
        Ok(handle) => handle,
        Err(err)   => { eprintln!("Could not open log file '{}': {}", config.log_path.display(), err); std::process::exit(1); }
    };
    WriteLogger::new(LevelFilter::Debug, Default::default(), handle);
    info!("Todo-Rust Client v{}", env!("CARGO_PKG_VERSION"));

    // Switch on the subcommand used
    match config.subcommand {
        ConfigSubcommand::Login{ host, username, password, identity_file } => {
            info!("Attempting to connect to '{}'...", &host);
        },

        ConfigSubcommand::Run{ host: _ } => {
            // Create a new TerminalUi instance.
            let mut tui = TerminalUi::default();
            if let Err(err) = tui.render_ui() { eprintln!("ERROR: {}", err); std::process::exit(1); };

            // Wait three seconds for the lolz
            std::thread::sleep(std::time::Duration::new(3, 0));
        },
    }
}
