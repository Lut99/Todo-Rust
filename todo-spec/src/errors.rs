/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   18 Mar 2022, 16:04:08
 * Last edited:
 *   18 Mar 2022, 16:35:35
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors in the todo-spec crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Errors that occur while working with credentials.
#[derive(Debug)]
pub enum CredentialError {
    /// Could not hash the given password.
    PasswordHashError{ err: argonautica::Error },

    /// The given credential is not valid to check as a password
    VerifyPasswordWrongCredential{ got: &'static str },
}

impl Display for CredentialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            CredentialError::PasswordHashError{ err } => write!(f, "Could not hash password: {}", err),

            CredentialError::VerifyPasswordWrongCredential{ got } => write!(f, "Cannot verify credential of type {} as a password", got),
        }
    }
}

impl Error for CredentialError {}
