/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   16 Mar 2022, 18:02:45
 * Last edited:
 *   17 Mar 2022, 10:14:16
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





/***** LIBRARY STRUCTS *****/
/// Defines the config-file part of the Config struct.
#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    /// Defines the default host to connect to.
    host : Option<Url>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            host : None,
        }
    }
}



/// Defines the configurable part for the client-side todo tool.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// The location of the config file
    #[clap(short, long, default_value = &DEFAULT_CONFIG_PATH, help = "The location of the config file for the client.")]
    pub config_path : PathBuf,

    /// Overrides the remote host to connect to.
    #[clap(long, help = "The remote host to connect to. If omitted, uses the value specified in the configuration file (see 'login').")]
    pub host : Option<url::Url>,
}

impl Config {
    /// Loads the config file by first populating it with command-line arguments and then with the given config file.
    /// 
    /// **Returns**  
    /// A new Config instance on success, or else an Error.
    pub fn load() -> Result<Self, Error> {
        // First, parse the CLI
        let mut config = Self::parse();

        // Next, open the config file and parse it as the correct struct
        let handle = match File::open(&config.config_path) {
            Ok(handle) => handle,
            Err(err)   => {
                // If it's not-found, we generate it first
                if err.kind() == std::io::ErrorKind::NotFound {
                    // Make sure the path exists
                    if let Err(err) = fs::create_dir_all(&config.config_path.parent().expect("Config path does not have a parent-part; this should never happen!")) {
                        return Err(Error::DirCreateError{ path: config.config_path, err });
                    }

                    // Try to open the file
                    let handle = match File::create(&config.config_path) {
                        Ok(handle) => handle,
                        Err(err)   => { return Err(Error::FileCreateError{ path: config.config_path, err }); }
                    };

                    // Write to it with serde
                    if let Err(err) = serde_json::to_writer_pretty(handle, &ConfigFile::default()) {
                        return Err(Error::FileGenerateError{ path: config.config_path, err });
                    }

                    // Now, open the same handle again to continue
                    match File::open(&config.config_path) {
                        Ok(handle) => handle,
                        Err(err)   => { return Err(Error::FileOpenError{ path: config.config_path, err }); }
                    }
                } else {
                    return Err(Error::FileOpenError{ path: config.config_path, err });
                }
            }
        };
        let reader = BufReader::new(handle);
        let config_file: ConfigFile = match serde_json::from_reader(reader) {
            Ok(file) => file,
            Err(err) => { return Err(Error::FileParseError{ path: config.config_path, err }); }
        };

        // Overwrite the relevant parts of the struct
        if let None = config.host {
            config.host = config_file.host.map(|host| host.0);
        }

        // Done!
        Ok(config)
    }
}
