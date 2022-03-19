/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 11:45:08
 * Last edited:
 *   19 Mar 2022, 12:26:26
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the implementation of the authorization service.
 *   
 *   The service takes some kind of credential (specified in todo-spec), and
 *   uses that to authorize the user. Then, on success, it generates a JWT,
 *   which is used by the other services to verify the user.
 * 
 *   This server uses a backend MySQL server to store the relevant user data.
**/

use std::net::Ipv4Addr;

use clap::Parser;
use log::{info, debug, LevelFilter};
use simplelog::{ColorChoice, TerminalMode, TermLogger};
use warp::Filter;

use todo_auth::login;


/***** ARGUMENTS *****/
/// Defines the command-line arguments available for the auth service.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// If given, shows debug prints as well
    #[clap(short, long, help = "If given, also logs debug prints.")]
    debug : bool,

    /// The hostname to use for setting up the server
    #[clap(short, long, help = "The hostname to bind the server to. Use '0.0.0.0' to accept anything.", default_value = "0.0.0.0")]
    host : Ipv4Addr,
    /// The port to bind the server to
    #[clap(short, long, help = "The port to bind the server to.", default_value = "4242")]
    port : u16,
}





/***** ENTRYPOINT *****/
#[tokio::main]
async fn main() {
    // Read command-line arguments
    let args = Arguments::parse();

    // Setup the logger
    if let Err(err) = TermLogger::init(if args.debug { LevelFilter::Debug } else { LevelFilter::Info }, Default::default(), TerminalMode::Mixed, ColorChoice::Auto) {
        eprintln!("Could not initialize logger: {}", err);
        std::process::exit(1);
    }
    info!("Todo-Rust Authorization Service v{}", env!("CARGO_PKG_VERSION"));



    // Prepare the warp filter for logging in
    debug!("Preparing warp filter for 'v1/login'...");
    let users = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(login::handle);

    // Prepare the global filter
    debug!("Preparing global warp filter...");
    let filter = users;

    // Run the server
    info!("Running warp server @ {}:{}", &args.host, &args.port);
    warp::serve(filter)
        .run((args.host, args.port))
        .await;
}
