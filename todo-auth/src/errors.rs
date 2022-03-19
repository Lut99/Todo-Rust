/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 11:48:04
 * Last edited:
 *   19 Mar 2022, 20:17:00
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects the errors for the todo-auth package.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use url::Url;
use warp::reject::Reject;


/***** ERRORS *****/
/// Defines top-level errors.
#[derive(Debug)]
pub enum AuthError {
    /// Something went wrong with a credential
    CredentialError{ err: todo_spec::credentials::Error },
    /// Could not match the given two credentials
    CredentialVerifyError{ err: todo_spec::credentials::Error },
    /// The given root credentials are outdated
    RootCredentialsOutdated,

    /// Could not create the connection pool
    MySqlPoolCreateError{ url: Url, err: mysql::Error },
    /// Could not connect to the local MySQL database
    MySqlConnectError{ err: mysql::Error },
    /// Could not execute the given query
    MySqlQueryError{ query: String, err: mysql::Error },
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            AuthError::CredentialError{ err }       => write!(f, "{}", err),
            AuthError::CredentialVerifyError{ err } => write!(f, "Could not verify credentials: {}", err),
            AuthError::RootCredentialsOutdated      => write!(f, "The provided root credentials are outdated; update the file or re-generate the database"),

            AuthError::MySqlPoolCreateError{ url, err } => write!(f, "Could not create a MySQL connection pool to database @ {}: {}", url, err),
            AuthError::MySqlConnectError{ err }         => write!(f, "Could not connect to MySQL database: {}", err),
            AuthError::MySqlQueryError{ query, err }    => write!(f, "Could not execute query '{}': {}", query, err),
        }
    }
}

impl Error for AuthError {}



/// Defines the errors that may occur during login
#[derive(Debug)]
pub enum LoginError {
    /// Could not connect to the local MySQL database
    MySqlConnectError{ err: mysql::Error },
    /// Could not execute the given query
    MySqlQueryError{ query: String, err: mysql::Error },

    /// Something went wrong with a credential
    CredentialError{ err: todo_spec::credentials::Error },
    /// Could not match the given two credentials
    CredentialVerifyError{ err: todo_spec::credentials::Error },
    /// The given user was not known to the system
    UnknownUser{ username: String },
}

impl Display for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            LoginError::MySqlConnectError{ err }      => write!(f, "Could not connect to MySQL database: {}", err),
            LoginError::MySqlQueryError{ query, err } => write!(f, "Could not execute query '{}': {}", query, err),

            LoginError::CredentialError{ err }       => write!(f, "{}", err),
            LoginError::CredentialVerifyError{ err } => write!(f, "Could not verify credentials: {}", err),
            LoginError::UnknownUser{ username }      => write!(f, "Unknown user '{}'", username),
        }
    }
}

impl Error for LoginError {}

impl Reject for LoginError {}
