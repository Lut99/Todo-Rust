/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   16 Mar 2022, 18:01:21
 * Last edited:
 *   19 Mar 2022, 12:26:18
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint for the client-side binary of the Todo tool.
**/

use std::fs::File;

use log::{error, info};
use simplelog::{LevelFilter, WriteLogger};
use todo_client::cli::{Action, Config};
use todo_client::tui::TerminalUi;


/***** ENTRYPOINT *****/
fn main() {
    // Prepare the config
    let (args, file) = match Config::load() {
        Ok(res)  => res,
        Err(err) => { eprintln!("Could not load configuration: {}", err); std::process::exit(1); }
    };
    let config = match Config::combine(args, file) {
        Ok(config) => config,
        Err(err)   => { eprintln!("{}", err); std::process::exit(1); }
    };

    // Setup the logger
    let handle = match File::create(&config.log_path) {
        Ok(handle) => handle,
        Err(err)   => { eprintln!("Could not open log file '{}': {}", config.log_path.display(), err); std::process::exit(1); }
    };
    if let Err(err) = WriteLogger::init(LevelFilter::max(), Default::default(), handle) {
        eprintln!("Could not initialize logger: {}", err);
        std::process::exit(1);
    };
    info!("Todo-Rust Client v{}", env!("CARGO_PKG_VERSION"));

    // Switch on the subcommand used
    match config.action {
        Action::Login{ host, credential } => {
            info!("Attempting to connect to '{}' with authentication method {}...", &host, credential.typename());
            

        },

        Action::Run{ host: _ } => {
            // Create a new TerminalUi instance.
            let mut tui = TerminalUi::default();
            if let Err(err) = tui.render_ui() { error!("{}", err); std::process::exit(1); };

            // Wait three seconds for the lolz
            std::thread::sleep(std::time::Duration::new(3, 0));
        },
    }
}
