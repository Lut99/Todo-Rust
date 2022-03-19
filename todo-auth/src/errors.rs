/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 11:48:04
 * Last edited:
 *   19 Mar 2022, 12:15:55
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects the errors for the todo-auth package.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines the errors that may occur during login
#[derive(Debug)]
pub enum LoginError {
    /// Could not parse the input as the LoginJson
    LoginParseError{ err: serde_json::Error },
}

impl Display for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            LoginError::LoginParseError{ err } => write!(f, "Could not parse login JSON: {}", err),
        }
    }
}

impl Error for LoginError {}
