/* CREDENTIALS.rs
 *   by Lut99
 *
 * Created:
 *   17 Mar 2022, 18:35:32
 * Last edited:
 *   19 Mar 2022, 15:08:36
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the common credential definitions and logic across both the
 *   client and the server.
**/

use std::fs::File;
use std::io::Read;
use std::path::Path;

use argon2::{self, Config};
use rand::RngCore;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

pub use crate::errors::CredentialError as Error;


/***** UNIT TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_simple() {
        // First, try to make a valid Password credential
        let cred = match Credential::from_password("john", "this_is_secret!") {
            Ok(cred) => cred,
            Err(err) => { panic!("from_password() should not have crashed, but it returned: {}", err); }
        };
        #[allow(irrefutable_let_patterns)]
        if let Credential::Password(user, pass) = cred {
            assert_eq!(user, String::from("john"));
            // Check if any hashing has occurred (due to salts we can't check which)
            assert_ne!(pass, String::from("this_is_a_secret!"));
        } else {
            panic!("from_password() returned a non-password {} credential", cred.typename());
        }
    }

    #[test]
    fn test_password_verify() {
        // Try to make a valid Password credential
        let cred = match Credential::from_password("john", "this_is_secret!") {
            Ok(cred) => cred,
            Err(err) => { panic!("from_password() should not have crashed, but it returned: {}", err); },
        };

        // Now match it with a matching username/password
        match cred.verify_password("john", "this_is_secret!") {
            Ok(res)  => { assert_eq!(res, true); },
            Err(err) => { panic!("verify_password() should not have crashed, but it returned: {}", err); },
        }
        // Now match it with a not-matching username
        match cred.verify_password("john2", "this_is_secret!") {
            Ok(res)  => { assert_eq!(res, false); },
            Err(err) => { panic!("verify_password() should not have crashed, but it returned: {}", err); },
        }
        // Now match it with a not-matching password
        match cred.verify_password("john", "this_is_secret?") {
            Ok(res)  => { assert_eq!(res, false); },
            Err(err) => { panic!("verify_password() should not have crashed, but it returned: {}", err); },
        }
    }

    #[test]
    fn test_password_salt() {
        // Make two hashes of the same password
        let cred1 = match Credential::from_password("john", "this_is_secret!") {
            Ok(cred) => cred,
            Err(err) => { panic!("from_password() should not have crashed, but it returned: {}", err); }
        };
        let cred2 = match Credential::from_password("john", "this_is_secret!") {
            Ok(cred) => cred,
            Err(err) => { panic!("from_password() should not have crashed, but it returned: {}", err); }
        };

        // Make sure that the hashes are _not_ the same
        #[allow(irrefutable_let_patterns)]
        if let (Credential::Password(_, pass1), Credential::Password(_, pass2)) = (&cred1, &cred2) {
            assert_ne!(pass1, pass2);
        } else {
            panic!("from_password() returned non-password ({}, {}) credential(s)", cred1.typename(), cred2.typename());
        }
    }

    #[test]
    fn test_illegal_username() {
        // Make a hash with a username that is illegal
        let _ = match Credential::from_password("john#$", "this_is_secret!") {
            Ok(_)    => { panic!("from_password() should have crashed due to illegal username, but it didn't"); },
            Err(err) => {
                match err {
                    Error::InvalidUsername{ username: _ } => {},
                    err => { panic!("from_password() crashed, but not because of an illegal username: {}", err); }
                }
            },
        };
    }
}





/***** CONSTANTS *****/
/// Defines the regular expression that is used to match usernames.
const USERNAME_REGEX: &str = r"^[0-9a-zA-Z_-]+$";
/// The length of the salt used
const SALT_SIZE: usize = 128;





/***** LIBRARY STRUCTS *****/
/// Defines one of multiple types of Credentials.
#[derive(Debug, Eq, PartialEq)]
pub enum Credential {
    /// The user has given us an (already hashed) password.
    Password(String, String),
}

impl Credential {
    /// Constructor for the Credential that takes a plain-text password and hashes it.
    /// 
    /// **Generic types**
    ///  * `S`: The String-like type of the username that is passed to the function.
    ///  * `B`: The Bytes-like type of the password that is passed to the function.
    /// 
    /// **Arguments**
    ///  * `username`: The username of the user to whom the given password belongs.
    ///  * `password`: The plain text password we would like to hash as fast as possible.
    /// 
    /// **Returns**  
    /// The new Credential instance on success, or else an Error.
    pub fn from_password<S: Into<String>, B: AsRef<[u8]>>(username: S, password: B) -> Result<Self, Error> {
        // Convert String-like into String
        let username = username.into();
        // Convert bytes-like into bytes
        let password = password.as_ref();

        // Verify that the username contains no illegal characters
        let re = Regex::new(USERNAME_REGEX).expect("Illegal Regex for matching usernames; this should never happen!");
        if !re.is_match(&username) { return Err(Error::InvalidUsername{ username }); }

        // Generate a random salt
        let mut rng = rand::thread_rng();
        let mut salt: Vec<u8> = vec![0; SALT_SIZE];
        rng.fill_bytes(&mut salt);

        // Hash with the rust-argon crate
        let config = Config::default();
        let hash = match argon2::hash_encoded(password, &salt, &config) {
            Ok(hash) => hash,
            Err(err) => { return Err(Error::PasswordHashError{ err }); }
        };

        // Create a new Credential with this hash
        Ok(Credential::Password(username, hash))
    }

    /// Constructor for the Credential that loads a username/hashed password pair from disk.
    /// 
    /// **Generic types**
    ///  * `P`: The Path-like type of the path that is passed to the function.
    /// 
    /// **Arguments**
    ///  * `path`: A path-like that refers to the file where the username and hashed password is stored.
    /// 
    /// **Returns**  
    /// The new Credential instance on success, or else an Error.
    pub fn from_password_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Convert Path-like into Path
        let path: &Path = path.as_ref();

        // Try to open the file
        let mut handle = match File::open(path) {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::FileOpenError{ path: path.to_path_buf(), err }); }
        };

        // Try to read everything in the file
        let mut data = String::new();
        if let Err(err) = handle.read_to_string(&mut data) {
            return Err(Error::FileReadError{ path: path.to_path_buf(), err });
        }

        // Use the 'from_string' constructor to split the string and deal with it
        drop(handle);
        Self::from_password_string(data)
    }

    /// Constructor for the Credential that takes a given username/password string and parses it.
    /// 
    /// **Generic types**
    ///  * `S`: The String-like type of the serialized username/password pair that is passed to the function.
    /// 
    /// **Arguments**
    ///  * `string`: A String-like that constains the serialized username/password pair we will base a new Credential on.
    /// 
    /// **Returns**  
    /// The new Credential instance on success, or else an Error.
    pub fn from_password_string<S: Into<String>>(string: S) -> Result<Self, Error> {
        // Convert the String-like into a String
        let string: String = string.into();

        // Try to split the string on the first '+' we see
        let mut found = false;
        let mut username = String::with_capacity(string.len());
        let mut password = String::with_capacity(string.len());
        for c in string.graphemes(true) {
            if !found && c == "+" {
                found = true;
            } else if !found {
                username.push_str(c);
            } else {
                password.push_str(c);
            }
        }
        // Check we found one
        if !found { return Err(Error::MissingSeparator); }

        // Make sure the username is legal
        let re = Regex::new(USERNAME_REGEX).expect("Illegal Regex for matching usernames; this should never happen!");
        if !re.is_match(&username) { return Err(Error::InvalidUsername{ username }); }

        // That's it! Now return it
        username.shrink_to_fit();
        password.shrink_to_fit();
        Ok(Credential::Password(username, password))
    }



    /// Compares this Credential to the given username + password.
    /// 
    /// **Generic types**
    ///  * `S`: The String-like type of the username that is passed to the function.
    ///  * `B`: The Bytes-like type of the password that is passed to the function.
    /// 
    /// **Arguments**
    ///  * `username`: The username of the user to whom the given password belongs.
    ///  * `password`: The plain text password we would like to hash as fast as possible.
    /// 
    /// **Returns**  
    /// Whether or not the passwords match, or else an Error if some error occurred while hashing.
    pub fn verify_password<S: Into<String>, B: AsRef<[u8]>>(&self, username: S, password: B) -> Result<bool, Error> {
        // Convert String-like into String
        let username = username.into();
        // Convert bytes-like into bytes
        let password = password.as_ref();

        // Make sure we are a Password credential
        #[allow(irrefutable_let_patterns)]
        if let Credential::Password(self_username, self_password) = self {
            // First, make sure the username makes sense
            if self_username != &username { return Ok(false); }

            // Verify the hash
            let is_valid = match argon2::verify_encoded(&self_password, &password) {
                Ok(is_valid) => is_valid,
                Err(err)     => { return Err(Error::PasswordHashError{ err }); }
            };

            // Done; return the success state
            Ok(is_valid)
        } else {
            Err(Error::VerifyPasswordWrongCredential{ got: self.typename() })
        }
    }



    /// Returns the typename of the Credential.
    #[inline]
    pub fn typename(&self) -> &'static str {
        match self {
            Credential::Password(_, _) => "Password",
        }
    }
}
