/* LOGIN.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 12:05:59
 * Last edited:
 *   19 Mar 2022, 12:28:13
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Handles the logging-in part of the authorization service.
**/

use warp::{Rejection, Reply};
use warp::http::StatusCode;

use todo_spec::api::LoginJson;

pub use crate::errors::LoginError as Error;


/***** LIBRARY FUNCTIONS *****/
/// Handles the logging-in part of the authorization service.
pub async fn handle(body: LoginJson) -> Result<impl Reply, Rejection> {
    // Return the username
    Ok(warp::reply::with_status(
        format!("Username: {}\nPassword: {}\n", body.username, body.password),
        StatusCode::OK,
    ))
}
