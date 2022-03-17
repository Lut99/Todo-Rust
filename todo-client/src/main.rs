/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   16 Mar 2022, 18:01:21
 * Last edited:
 *   17 Mar 2022, 10:05:28
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint for the client-side binary of the Todo tool.
**/

use todo_client::cli::Config;
use todo_client::tui::TerminalUi;


/***** ENTRYPOINT *****/
fn main() {
    // Parse the arguments
    let settings = match Config::load() {
        Ok(settings) => settings,
        Err(err)     => { eprintln!("ERROR: {}", err); std::process::exit(1); }
    };

    // Create a new TerminalUi instance.
    let tui = TerminalUi::default();



    // 
}
