/* LOGIN.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 12:05:59
 * Last edited:
 *   19 Mar 2022, 21:19:00
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Handles the logging-in part of the authorization service.
**/

use std::ops::Add;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use log::{debug, error, info};
use mysql::Pool;
use mysql::prelude::Queryable;
use sha2::Sha256;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use warp::{Rejection, Reply};
use warp::http::StatusCode;

use todo_spec::api::LoginJson;
use todo_spec::credentials::Credential;

pub use crate::errors::LoginError as Error;
pub use crate::spec::{Account, JWT_EXPIRATION_TIME};


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
///  * `secret`: The server-wide shared secret that is used to sign the JWT's.
///  * `body`: The message body we got with the request.
/// 
/// **Returns**  
/// The Warp reply on success, or a Warp rejection on failure.
pub async fn handle(pool: Arc<Pool>, secret: Arc<String>, body: LoginJson) -> Result<impl Reply, Rejection> {
    // Try to connect to the MySQL database
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(err) => { throw!(Error::MySqlConnectError{ err }); }
    };

    // Select the appropriate database
    debug!("Selecting database...");
    let query = String::from("USE todo;");
    if let Err(err) = conn.query_drop(&query) { throw!(Error::MySqlQueryError{ query, err }); };

    // Query the database for this username
    debug!("Searching for user '{}'...", &body.username);
    let query = format!("SELECT id, name, pass FROM users WHERE name = '{}';", &body.username);
    let users: Vec<Account> = match conn.query_map(
        &query,
        |(id, name, pass)| { Account{ id, credential : Credential::new::<String, String>(name, pass).expect("Invalid username made its way into the MySQL database; this should never happen!") } }
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
    debug!("User '{}' login success; generating JWT", user.credential.user());

    // Define when the JWT will expire
    let expiration_date: OffsetDateTime = OffsetDateTime::now_utc().add(Duration::from_secs(JWT_EXPIRATION_TIME));

    // Create the Hmac key first
    let key: Hmac<Sha256> = match Hmac::new_from_slice(secret.as_bytes()) {
        Ok(key)  => key,
        Err(err) => { throw!(Error::HmacKeyError{ err }); }
    };

    // Define the claims (i.e., content) that we'll carry in the JWT
    let mut claims = BTreeMap::new();
    claims.insert("id", format!("{}", user.id));
    claims.insert("exp", expiration_date.format(&Rfc3339).expect("Could not format JWT expiration date to ISO/RFC3339; this should never happen!"));

    // We generate a JWT
    let token = match claims.sign_with_key(&key) {
        Ok(token) => token,
        Err(err)  => { throw!(Error::JwtCreateError{ err }); }
    };

    // Return the token!
    info!("User '{}' authorized with JWT until {}", user.credential.user(), expiration_date.format(&Rfc3339).expect("Could not format JWT expiration date to ISO/RFC3339; this should never happen!"));
    Ok(warp::reply::with_status(
        token,
        StatusCode::OK,
    ))
}
