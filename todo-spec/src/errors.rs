/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   18 Mar 2022, 16:04:08
 * Last edited:
 *   19 Mar 2022, 09:43:18
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors in the todo-spec crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;


/***** ERRORS *****/
/// Errors that occur while working with credentials.
#[derive(Debug)]
pub enum CredentialError {
    /// The given username was not valid.
    InvalidUsername{ username: String },
    /// Could not hash the given password.
    PasswordHashError{ err: argonautica::Error },

    /// Could not open the given file
    FileOpenError{ path: PathBuf, err: std::io::Error },
    /// Could not read from the given file
    FileReadError{ path: PathBuf, err: std::io::Error },

    /// Didn't find the split '+' in the username/character pair.
    MissingSeparator,

    /// The given credential is not valid to check as a password
    VerifyPasswordWrongCredential{ got: &'static str },
}

impl Display for CredentialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            CredentialError::InvalidUsername{ username } => write!(f, "Given username '{}' is invalid; it may only contain alphanumerical characters and underscores (_) and dashes(-)", username),
            CredentialError::PasswordHashError{ err }    => write!(f, "Could not hash password: {}", err),

            CredentialError::FileOpenError{ path, err } => write!(f, "Could not open file '{}': {}", path.display(), err),
            CredentialError::FileReadError{ path, err } => write!(f, "Could not read from file '{}': {}", path.display(), err),

            CredentialError::MissingSeparator => write!(f, "Missing username/password separator '+' in serialized username/password pair"),

            CredentialError::VerifyPasswordWrongCredential{ got } => write!(f, "Cannot verify credential of type {} as a password", got),
        }
    }
}

impl Error for CredentialError {}