/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 11:45:08
 * Last edited:
 *   19 Mar 2022, 21:59:27
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

use std::fs;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use log::{info, debug, error, LevelFilter};
use mysql::{Opts, Pool, PooledConn};
use mysql::prelude::Queryable;
use simplelog::{ColorChoice, TerminalMode, TermLogger};
use url::Url;
use warp::Filter;

use todo_spec::credentials::Credential;

use todo_auth::login;
use todo_auth::errors::AuthError as Error;
use todo_auth::spec::Account;


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

    /// The mysql server to connect to
    #[clap(short, long, help = "The address:port of the MySQL server to connect to.", env = "MYSQL_URL")]
    mysql_url : Url,
    /// The location of the root MySQL database password file
    #[clap(long, help = "The location of the MySQL database's root password.", default_value = "./mysql_root.cred")]
    mysql_root_cred : PathBuf,
    /// The location of the root user file
    #[clap(short, long, help = "The location of the root user's credentials in the Todo server.", default_value = "./root.cred")]
    root_cred : PathBuf,
    /// The location of the JWT secret file
    #[clap(short, long, help = "The location of the JWT secret file.", default_value = "./jwt.secret")]
    secret: PathBuf,
}





/***** HELPER FUNCTIONS *****/
/// Ensure the desired database structure is present in the MySQL database.
/// 
/// **Arguments**
///  * `pool`: The MySQL pool to allocate a new connection from.
///  * `root_cred`: The credentials for the root account. Will be used to initiate it if the root does not exist, or checked to see if the root authentication is still valid.
/// 
/// **Returns**  
/// Nothing on success, or else an Error.
fn ensure_database(pool: Arc<Pool>, root_cred: &Credential) -> Result<(), Error> {
    // Connect to the database to setup tables
    info!("Preparig database...");
    debug!("Connecting to MySQL database...");
    let mut conn: PooledConn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(err) => { return Err(Error::MySqlConnectError{ err }); }
    };

    // Create the database if it does not yet exist
    debug!("Creating 'todo' database if necessary...");
    let query = String::from("CREATE DATABASE IF NOT EXISTS todo; USE DATABASE todo;");
    if let Err(err) = conn.query_drop(&query) {
        return Err(Error::MySqlQueryError{ query, err });
    };

    // Select the database
    debug!("Selecting 'todo' database...");
    let query = String::from("USE todo;");
    if let Err(err) = conn.query_drop(&query) {
        return Err(Error::MySqlQueryError{ query, err });
    };



    // Create the users table if it does not yet exist
    debug!("Creating 'users' table if necessary...");
    let query = String::from(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL UNIQUE,
            pass VARCHAR(255) NOT NULL
        );"
    );
    if let Err(err) = conn.query_drop(&query) {
        return Err(Error::MySqlQueryError{ query, err });
    };

    // Insert the root user into it if it does not exist yet
    debug!("Checking if root user already exists...");
    let query = format!("SELECT id, name, pass FROM users WHERE name = '{}';", root_cred.user());
    let root_users: Vec<Account> = match conn.query_map(
        &query,
        |(id, name, pass)| { Account{ id, credential : Credential::new::<String, String>(name, pass).expect("Invalid username made its way into the MySQL database; this should never happen!") } }
    ) {
        Ok(res)  => res,
        Err(err) => { return Err(Error::MySqlQueryError{ query, err }); }
    };
    if root_users.len() == 0 {
        debug!("Inserting root user...");

        // Create the (double) hashed version of the password
        let root_cred = match Credential::from_plain(root_cred.user(), root_cred.pass()) {
            Ok(cred) => cred,
            Err(err) => { return Err(Error::CredentialError{ err }); }
        };

        // Write it to the database
        let query = format!("INSERT INTO users (name, pass) VALUES ('{}', '{}');", root_cred.user(), root_cred.pass());
        if let Err(err) = conn.query_drop(&query) {
            return Err(Error::MySqlQueryError{ query, err });
        };
    } else {
        debug!("Root already exists; verifying root credentials...");

        // Get the account
        let root: &Account = &root_users[0];

        // Match with the root credentials
        match root.credential.verify(root_cred.user(), root_cred.pass()) {
            Ok(res) => { if !res { return Err(Error::RootCredentialsOutdated); } },
            Err(err) => { return Err(Error::CredentialVerifyError{ err }); }
        }
    }



    // That's it for now
    debug!("Database preparation complete.");
    Ok(())
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



    // Load the credentials
    debug!("Loading MySQL credentials...");
    let mysql_cred = match Credential::from_file(args.mysql_root_cred) {
        Ok(cred) => cred,
        Err(err) => { error!("{}", err); std::process::exit(1); }  
    };

    debug!("Loading root credentials...");
    let root_cred = match Credential::from_file(args.root_cred) {
        Ok(cred) => cred,
        Err(err) => { error!("{}", err); std::process::exit(1); }
    };

    debug!("Loading JWT secret...");
    let secret = match fs::read_to_string(args.secret) {
        Ok(secret) => Arc::new(secret),
        Err(err)   => { error!("{}", err); std::process::exit(1); }
    };



    // Prepare the pool for local MySQL connections
    info!("Preparing connections to MySQL database @ {}...", &args.mysql_url);
    let pool = match Pool::new(Opts::from_url(&format!("mysql://{}:{}@{}", mysql_cred.user(), mysql_cred.pass(), args.mysql_url)).expect("Could not get URL from Url; this should never happen!")) {
        Ok(pool) => Arc::new(pool),
        Err(err) => { error!("{}", Error::MySqlPoolCreateError{ url: args.mysql_url, err }); std::process::exit(1); }
    };

    // Prepare the database if needed
    if let Err(err) = ensure_database(pool.clone(), &root_cred) {
        error!("{}", err);
        std::process::exit(1);
    };



    // Prepare the warp filter for logging in
    debug!("Preparing warp filter for 'v1/login'...");
    let tpool = pool.clone(); let tsecret = secret.clone();
    let login = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(move |body| { login::handle(tpool.clone(), tsecret.clone(), body) });

    // Prepare the warp filter for testing login
    debug!("Preparing warp filter for 'v1/login/test'...");
    let tpool = pool.clone();
    let login_test = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("login"))
        .and(warp::path("test"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(move |body| { login::handle_test(tpool.clone(), body) });

    // Prepare the global filter
    debug!("Preparing global warp filter...");
    let filter = login.or(login_test);

    // Run the server
    info!("Running warp server @ {}:{}", &args.host, &args.port);
    warp::serve(filter)
        .run((args.host, args.port))
        .await;
}
