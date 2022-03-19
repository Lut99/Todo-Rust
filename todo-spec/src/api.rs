/* API.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 12:11:12
 * Last edited:
 *   19 Mar 2022, 21:05:25
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines various structs that are used in the API communication between
 *   client and services.
**/

use serde::{Serialize, Deserialize};


/***** LIBRARY STRUCTS *****/
/// Defines the JSON for the login struct.
#[derive(Serialize, Deserialize)]
pub struct LoginJson {
    /// The username of the user
    pub username : String,
    /// The password of the user
    pub password : String,
}




/***** LIBRARY FUNCTIONS *****/

