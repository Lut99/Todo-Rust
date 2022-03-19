/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   16 Mar 2022, 18:02:45
 * Last edited:
 *   19 Mar 2022, 18:48:50
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the command-line slash config file reading part of the
 *   todo-client tool.
**/

use std::io::BufReader;
use std::fmt::{Display, Formatter, Result as FResult};
use std::fs::{self, File};
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};

use todo_spec::credentials::Credential;

pub use crate::errors::ConfigError as Error;


/***** LAZY CONSTANTS *****/
lazy_static! {
    /// The default config path
    static ref DEFAULT_CONFIG_PATH: String = format!("{}", dirs_2::config_dir().expect("Could not get standard user configuration directory").join("todo/config.json").display());
}





/***** HELPER STRUCTS *****/
/// Visitor for the url::Url class.
struct UrlVisitor;

impl<'de> Visitor<'de> for UrlVisitor {
    type Value = url::Url;

    fn expecting(&self, formatter: &mut Formatter) -> FResult {
        formatter.write_str("an URL")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        url::Url::from_str(value).map_err(|err| E::custom(format!("{}", err)))
    }
}



/// Wraps around the Url to make it serializeable.
#[derive(Debug)]
struct Url(url::Url);

impl Deref for Url {
    type Target = url::Url;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Url {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Url {
    type Err = url::ParseError;
    #[inline]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        url::Url::from_str(value).map(|res| Url(res))
    }
}

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        // Simply serialize as string
        serializer.serialize_str(&format!("{}", **self))
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>
    {
        // Simply deserialize as string
        deserializer.deserialize_str(UrlVisitor).map(|res| Url(res))
    }
}

impl Display for Url {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.0)
    }
}





/***** ARGUMENT STRUCTS *****/
/// Defines the command-line part of the Config struct.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// The location of the config file
    #[clap(short, long, default_value = &DEFAULT_CONFIG_PATH, help = "The location of the config file for the client.")]
    config_path : PathBuf,
    /// The location of the logging file
    #[clap(short, long, help = "The location of the logging file for the client.")]
    log_path : Option<PathBuf>,

    /// Defined the subcommand that is run
    #[clap(subcommand)]
    subcommand : ArgumentSubcommand,
}



/// Talks about the subcommands for the config.
#[derive(Debug, Parser)]
enum ArgumentSubcommand {
    /// A subcommand that generates a .cred file for the given login
    #[clap(name = "generate", about = "Generate a new credential file for the given username and authentication method.")]
    Generate {
        #[clap(short, long, help = "The location of the output file.", default_value = "./user.cred")]
        output : PathBuf,

        #[clap(help = "The username to login with.")]
        username : String,
        #[clap(short, long, help = "If given, tries to login using a password that is read from stdin.")]
        password : bool,
    },

    /// A Subcommand that logs the user in remotely
    #[clap(name = "login", about = "Login to a Todo server.")]
    Login {
        #[clap(help = "The hostname & port of the remote server to login to.")]
        host : url::Url,

        #[clap(help = "The username to login with.")]
        username : String,

        #[clap(short, long, help = "If given, tries to login using a password that is read from stdin.")]
        password : bool,
    },

    /// No subcommand is used
    #[clap(name = "run", about = "Runs the normal interface to the Todo tool.")]
    Run {
        /// Overrides the remote host to connect to.
        #[clap(long, help = "The remote host to connect to. If omitted, uses the value specified in the configuration file (see the 'login' subcommand).")]
        host : Option<url::Url>,
    },
}





/***** FILE STRUCTS *****/
/// Defines the config-file part of the Config struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    /// Defines the standard logging path
    log_path : PathBuf,

    /// Defines the default host to connect to.
    host : Option<Url>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            log_path : dirs_2::config_dir().expect("Could not get standard user configuration directory").join("todo/todo.log"),
            host : None,
        }
    }
}





/***** LIBRARY STRUCTS *****/
/// Defines subcommands at Config time.
#[derive(Debug)]
pub enum Action {
    /// The user wants to generate a credential file.
    Generate {
        /// The output to write to
        output     : PathBuf,
        /// The credentials to write
        credential : Credential,
    },

    /// The user wants to login somewhere remotely.
    Login {
        /// The hostname of the host to login to.
        host       : url::Url,
        /// The credentials to login
        credential : Credential,
    },

    /// The user wants to run the tool normally.
    Run {
        /// The hostname after config file / CLI parsing
        host : url::Url,  
    },
}





/// Joins the Arguments and the File together in one Config struct.
#[derive(Debug)]
pub struct Config {
    /// The location of the config file
    pub config_path : PathBuf,
    /// The location of the logging file
    pub log_path    : PathBuf,

    /// The subcommand that is run
    pub action : Action,
}

impl Config {
    /// Reads the command-line arguments and loads the config file.  
    /// Note that, to use either, call combine() on the resulting pair.
    /// 
    /// **Returns**  
    /// A new (Arguments, ConfigFile) pair on success, or else an Error.
    pub fn load() -> Result<(Arguments, ConfigFile), Error> {
        // First, parse the CLI
        let args = Arguments::parse();

        // Next, open the config file and parse it as the correct struct
        let handle = match File::open(&args.config_path) {
            Ok(handle) => handle,
            Err(err)   => {
                // If it's not-found, we generate it first
                if err.kind() == std::io::ErrorKind::NotFound {
                    // Make sure the path exists
                    if let Err(err) = fs::create_dir_all(&args.config_path.parent().expect("Config path does not have a parent-part; this should never happen!")) {
                        return Err(Error::DirCreateError{ path: args.config_path, err });
                    }

                    // Try to open the file
                    let handle = match File::create(&args.config_path) {
                        Ok(handle) => handle,
                        Err(err)   => { return Err(Error::FileCreateError{ path: args.config_path, err }); }
                    };

                    // Write to it with serde
                    if let Err(err) = serde_json::to_writer_pretty(handle, &ConfigFile::default()) {
                        return Err(Error::FileGenerateError{ path: args.config_path, err });
                    }

                    // Now, open the same handle again to continue
                    match File::open(&args.config_path) {
                        Ok(handle) => handle,
                        Err(err)   => { return Err(Error::FileOpenError{ path: args.config_path, err }); }
                    }
                } else {
                    return Err(Error::FileOpenError{ path: args.config_path, err });
                }
            }
        };
        let reader = BufReader::new(handle);
        let file: ConfigFile = match serde_json::from_reader(reader) {
            Ok(file) => file,
            Err(err) => { return Err(Error::FileParseError{ path: args.config_path, err }); }
        };

        // Done!
        Ok((args, file))
    }

    /// Combines a loaded Arguments and ConfigFile struct into one Config.
    /// 
    /// **Arguments**
    ///  * `args`: The Argument struct to overwrite the data in the ConfigFile with.
    ///  * `file`: The ConfigFile that contains the data to fallback to if it's missing from the Arguments.
    /// 
    /// **Returns**  
    /// A new Config instance on success, or else an Error.
    pub fn combine(args: Arguments, file: ConfigFile) -> Result<Self, Error> {
        // Resolve the toplevel arguments in the config first
        let config_path = args.config_path;
        let log_path    = args.log_path.unwrap_or(file.log_path);

        // Next, match on the Arguments' subcommand to make an Action
        let action = match args.subcommand {
            ArgumentSubcommand::Generate{ output, username, password } => {
                // Decide what method of authentication to use
                let cred: Credential;
                if password {
                    // Prompt the user for a password
                    let password1 = match rpassword::prompt_password("Credential password:") {
                        Ok(password) => password,
                        Err(err)     => { return Err(Error::PasswordPromptError{ err }); }  
                    };
                    // Prompt the user for a password to verify
                    let password2 = match rpassword::prompt_password("Verify password:") {
                        Ok(password) => password,
                        Err(err)     => { return Err(Error::PasswordPromptError{ err }); }  
                    };

                    // Be sure they are the same
                    if password1 != password2 { return Err(Error::UnmatchingPasswords); }

                    // Create a Credential from it
                    cred = match Credential::from_plain(username, password1) {
                        Ok(cred) => cred,
                        Err(err) => { return Err(Error::CredentialError{ err }); }
                    };
                } else {
                    return Err(Error::NoCredentials);
                }

                // Create the action with that
                Action::Generate{ output, credential: cred }
            },

            ArgumentSubcommand::Login{ host, username, password } => {
                // Decide what method of authentication to use
                let cred: Credential;
                if password {
                    // Prompt the user for a password
                    let password = match rpassword::prompt_password(format!("Password for '{}':", &host)) {
                        Ok(password) => password,
                        Err(err)     => { return Err(Error::PasswordPromptError{ err }); }  
                    };

                    // Create a Credential from it
                    cred = match Credential::from_plain(username, password) {
                        Ok(cred) => cred,
                        Err(err) => { return Err(Error::CredentialError{ err }); }
                    };
                } else {
                    return Err(Error::NoCredentials);
                }

                // With the user having provided us their credentials, build the action
                Action::Login{ host, credential: cred }
            },

            ArgumentSubcommand::Run{ host } => {
                // Resolve the host
                let host = match host {
                    Some(host) => host,
                    None => match file.host {
                        Some(host) => host.0,
                        None       => { return Err(Error::NotLoggedIn); }
                    }
                };

                // Use that to build the action
                Action::Run{ host }
            },
        };

        // Now, build the action with the created parameters
        Ok(Self {
            config_path,
            log_path,
            
            action,
        })
    }
}
