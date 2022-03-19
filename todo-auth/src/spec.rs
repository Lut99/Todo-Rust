/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 15:35:26
 * Last edited:
 *   19 Mar 2022, 18:41:53
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains authorization-local specifications.
**/

use todo_spec::credentials::Credential;


/***** LIBRARY STRUCTS *****/
/// Defines a stored account in the database.
#[derive(Debug, Eq, PartialEq)]
pub struct Account {
    /// The credentials of the user.
    pub credential : Credential,
}
