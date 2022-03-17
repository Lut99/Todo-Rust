/* CREDENTIALS.rs
 *   by Lut99
 *
 * Created:
 *   17 Mar 2022, 18:35:32
 * Last edited:
 *   17 Mar 2022, 18:44:41
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the common credential definitions and logic across both the
 *   client and the server.
**/

use argon2::password_hash::PasswordHash;


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
pub enum Credentials<'a> {
    /// The user has given us an (already hashed) password.
    Password(PasswordHash<'a>),
    /// The user has given us an SSH key.
    IdentityFile(String),
}

impl<'a> Credentials<'a> {
    /// Constructor for the Credentials that takes a plain-text password and hashes it.
    /// 
    /// **Arguments**
    ///  * `password`: The plain text password we would like to hash as fast as possible.
    pub fn from_password(password: &'a str) -> Self {
        
    }

}
