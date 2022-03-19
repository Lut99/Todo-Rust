/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   17 Mar 2022, 09:26:00
 * Last edited:
 *   19 Mar 2022, 21:42:23
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects the errors in the client-side tool.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;

use reqwest::StatusCode;
use url::Url;

use todo_spec::credentials::Error as CredentialError;


/***** ERRORS *****/
/// Errors that occur while loading the Config struct.
#[derive(Debug)]
pub enum ConfigError {
    /// Could not create one or more of the configuration directories
    DirCreateError{ path: PathBuf, err: std::io::Error },
    /// Could not create a new config file.
    FileCreateError{ path: PathBuf, err: std::io::Error },
    /// Could not serialize the default config file OR write to the new config file.
    FileGenerateError{ path: PathBuf, err: serde_json::Error },

    /// Could not open the configuration file.
    FileOpenError{ path: PathBuf, err: std::io::Error },
    /// Could not parse the configuration file.
    FileParseError{ path: PathBuf, err: serde_json::Error },
    /// Could not update the configuration file.
    FileUpdateError{ path: PathBuf, err: serde_json::Error },

    /// THe user did not specify how they are going to give us credentials
    NoCredentials,
    /// Could not create a Credential struct
    CredentialError{ err: CredentialError },
    /// Could not prompt the user for a password
    PasswordPromptError{ err: std::io::Error },
    /// The first password and the second password asked do not match
    UnmatchingPasswords,

    /// The user was not logged in
    NotLoggedIn,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            ConfigError::DirCreateError{ path, err }    => write!(f, "Could not create config directory '{}': {}", path.display(), err),
            ConfigError::FileCreateError{ path, err }   => write!(f, "Could not create config file '{}': {}", path.display(), err),
            ConfigError::FileGenerateError{ path, err } => write!(f, "Could not write to new config file '{}': {}", path.display(), err),

            ConfigError::FileOpenError{ path, err }   => write!(f, "Could not open config file '{}': {}", path.display(), err),
            ConfigError::FileParseError{ path, err }  => write!(f, "Could not parse config file '{}': {}", path.display(), err),
            ConfigError::FileUpdateError{ path, err } => write!(f, "Could not update config file '{}': {}", path.display(), err),

            ConfigError::NoCredentials              => write!(f, "Did not specify a method to provide credentials"),
            ConfigError::CredentialError{ err }     => write!(f, "Could not create a Credential: {}", err),
            ConfigError::PasswordPromptError{ err } => write!(f, "Could not prompt for a password: {}", err),
            ConfigError::UnmatchingPasswords        => write!(f, "Passwords do not match; aborting"),

            ConfigError::NotLoggedIn => write!(f, "You are not logged-in; run the login subcommand first"),
        }
    }
}

impl Error for ConfigError {}



/// Errors that occur while working with the TUI.
#[derive(Debug)]
pub enum TuiError {
    /// Could not enable raw mode
    RawModeEnableError{ err: std::io::Error },
    /// Could not run the execute! macro
    ExecuteError{ err: std::io::Error },
    /// COuld not create the new Terminal instance
    TerminalCreateError{ err: std::io::Error },

    /// Could not disable raw mode
    RawModeDisableError{ err: std::io::Error },
    /// Could not show the terminal's cursor
    ShowCursorError{ err: std::io::Error },

    /// Could not draw to the terminal
    TerminalDrawError{ err: std::io::Error },
}

impl Display for TuiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            TuiError::RawModeEnableError{ err }  => write!(f, "Could not enable terminal raw mode: {}", err),
            TuiError::ExecuteError{ err }        => write!(f, "Could not run execute!: {}", err),
            TuiError::TerminalCreateError{ err } => write!(f, "Could not create new Terminal: {}", err),
            
            TuiError::RawModeDisableError{ err } => write!(f, "Could not disable terminal raw mode: {}", err),
            TuiError::ShowCursorError{ err }     => write!(f, "Could not show terminal cursor: {}", err),

            TuiError::TerminalDrawError{ err } => write!(f, "Could not draw to terminal: {}", err),
        }
    }
}

impl Error for TuiError {}



/// Errors that occur while logging in and such.
#[derive(Debug)]
pub enum LoginError {
    /// Could not serialize the Login request body
    SerializeError{ err: serde_json::Error },
    /// Could not append a path to a host URL
    UrlJoinError{ host: Url, path: String, err: url::ParseError },
    /// Could not send the login request.
    RequestError{ err: reqwest::Error },
    /// The server returned a non-valid response
    ResponseError{ status: StatusCode, response: String },
}

impl Display for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            LoginError::SerializeError{ err }           => write!(f, "Could not serialize the login request body: {}", err),
            LoginError::UrlJoinError{ host, path, err } => write!(f, "Could not append path '{}' to host '{}': {}", path, host, err),
            LoginError::RequestError{ err }             => write!(f, "Could not send login request: {}", err),
            LoginError::ResponseError{ status, response } => write!(f, "Host responded with status code {}{}\n\nResponse:\n{}\n", status.as_u16(), if status.canonical_reason().is_some() { format!(" ({})", status.canonical_reason().unwrap()) } else { String::new() }, response),
        }
    }
}

impl Error for LoginError {}
