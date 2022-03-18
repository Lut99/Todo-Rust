/* CREDENTIALS.rs
 *   by Lut99
 *
 * Created:
 *   17 Mar 2022, 18:35:32
 * Last edited:
 *   18 Mar 2022, 16:36:48
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the common credential definitions and logic across both the
 *   client and the server.
**/

use argonautica::{Hasher, Verifier};
use argonautica::input::Password;

pub use crate::errors::CredentialError as Error;


/***** UNIT TESTS *****/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}





/***** LIBRARY STRUCTS *****/
/// Defines one of multiple types of Credentials.
#[derive(Debug)]
pub enum Credential {
    /// The user has given us an (already hashed) password.
    Password(String),
}

impl Credential {
    /// Constructor for the Credential that takes a plain-text password and hashes it.
    /// 
    /// **Generic types**
    ///  * `P`: The type of the Password that is passed to the function.
    /// 
    /// **Arguments**
    ///  * `password`: The plain text password we would like to hash as fast as possible.
    /// 
    /// **Returns**  
    /// The new Credential instance on success, or else an Error.
    pub fn from_password<'a, P: Into<Password<'a>>>(password: P) -> Result<Self, Error> {
        // Build the hasher
        let mut hasher = Hasher::default();
        let hash = match hasher
            .opt_out_of_secret_key(true)
            .with_password(password)
            .hash()
        {
            Ok(hash) => hash,
            Err(err) => { return Err(Error::PasswordHashError{ err }); }
        };

        // Create a new Credential with this hash
        Ok(Credential::Password(hash))
    }

    

    /// Compares this Credential to the given password.
    /// 
    /// **Generic types**
    ///  * `P`: The type of the Password that is passed to the function.
    /// 
    /// **Arguments**  
    ///  * `password`: The plain text password we would like to verify.
    /// 
    /// **Returns**  
    /// Whether or not the passwords match, or else an Error if some error occurred while hashing.
    pub fn verify_password<'a, P: Into<Password<'a>>>(&self, password: P) -> Result<bool, Error> {
        // Make sure we are a Password credential
        #[allow(irrefutable_let_patterns)]
        if let Credential::Password(password_hash) = self {
             // Build the verifier
            let mut verifier = Verifier::default();
            let is_valid = match verifier
                .with_hash(password_hash)
                .with_password(password)
                .verify()
            {
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
            Credential::Password(_) => "Password",
        }
    }
}
