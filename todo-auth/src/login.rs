/* LOGIN.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 12:05:59
 * Last edited:
 *   19 Mar 2022, 17:28:35
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Handles the logging-in part of the authorization service.
**/

use std::sync::Arc;

use log::error;
use mysql::Pool;
use warp::{Rejection, Reply};
use warp::http::StatusCode;

use todo_spec::api::LoginJson;

pub use crate::errors::LoginError as Error;
pub use crate::spec::Account;


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
    let conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(err) => { error!("{}", Error::MySqlConnectError{ err }); return Err(warp::reject::reject()); }
    };

    // Query the database for this username
    

    // Return the username
    Ok(warp::reply::with_status(
        format!("Username: {}\nPassword: {}\n", body.username, body.password),
        StatusCode::OK,
    ))
}
