/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   17 Mar 2022, 09:26:00
 * Last edited:
 *   19 Mar 2022, 10:37:45
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects the errors in the client-side tool.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;

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
