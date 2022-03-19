/* LOGIN.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 12:05:59
 * Last edited:
 *   19 Mar 2022, 20:38:01
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Handles the logging-in part of the authorization service.
**/

use std::sync::Arc;

use hmac::Hmac;
use log::{debug, error};
use mysql::Pool;
use mysql::prelude::Queryable;
use warp::{Rejection, Reply};
use warp::http::StatusCode;

use todo_spec::api::LoginJson;
use todo_spec::credentials::Credential;

pub use crate::errors::LoginError as Error;
pub use crate::spec::Account;


/***** HELPER MACROS *****/
/// Writes the given error both to stderr via error!() and returns it as a custom reject
macro_rules! throw {
    ($err:expr) => {
        let err = $err;
        error!("{}", &err);
        return Err(warp::reject::custom(err));
    };
}





/***** LIBRARY FUNCTIONS *****/
/// Handles the logging-in part of the authorization service.
/// 
/// **Arguments**
///  * `pool`: The MySQL pool of connections to use for this service.
///  * `body`: The message body we got with the request.
/// 
/// **Returns**  
/// The Warp reply on success, or a Warp rejection on failure.
pub async fn handle(pool: Arc<Pool>, body: LoginJson) -> Result<impl Reply, Rejection> {
    // Try to connect to the MySQL database
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(err) => { throw!(Error::MySqlConnectError{ err }); }
    };

    // Query the database for this username
    debug!("Searching for user '{}'...", &body.username);
    let query = format!("SELECT name, pass FROM users WHERE name = '{}';", &body.username);
    let users: Vec<Account> = match conn.query_map(
        &query,
        |(name, pass)| { Account{ credential : Credential::new::<String, String>(name, pass).expect("Invalid username made its way into the MySQL database; this should never happen!") } }
    ) {
        Ok(res)  => res,
        Err(err) => { throw!(Error::MySqlQueryError{ query, err }); }
    };

    // Check if there are any
    if users.len() == 0 { return Ok(warp::reply::with_status(
        format!("Unknown username '{}'", body.username),
        StatusCode::NOT_FOUND,
    )); }
    let user: &Account = &users[0];

    // Verify the password
    match user.credential.verify(&body.username, &body.password) {
        Ok(is_valid) => {
            if !is_valid {
                return Ok(warp::reply::with_status(
                    "Invalid password".to_string(),
                    StatusCode::FORBIDDEN),
                );
            }
        },
        Err(err) => { throw!(Error::CredentialVerifyError{ err }); }
    }

    // Success! Generate a JWT for this user.

    // Return the username
    Ok(warp::reply::with_status(
        format!("Username: {}\nPassword: {}\n", body.username, body.password),
        StatusCode::OK,
    ))
}
