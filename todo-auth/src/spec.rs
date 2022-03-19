/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 15:35:26
 * Last edited:
 *   19 Mar 2022, 20:54:22
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains authorization-local specifications.
**/

use todo_spec::credentials::Credential;


/***** LIBRARY CONSTANTS *****/
/// The time (in seconds) that a JWT expires after it has been handed out
pub const JWT_EXPIRATION_TIME: u64 = 1 * 3600;





/***** LIBRARY STRUCTS *****/
/// Defines a stored account in the database.
#[derive(Debug, Eq, PartialEq)]
pub struct Account {
    /// The identifier of this user.
    pub id : u32,

    /// The credentials of the user.
    pub credential : Credential,
}
