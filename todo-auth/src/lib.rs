/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 11:47:45
 * Last edited:
 *   19 Mar 2022, 12:06:22
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the library entrypoint for the todo-auth application.
**/

/// Allows the use of the log crate macros
#[macro_use] extern crate log;

/// Collects all errors for this package.
pub mod errors;
/// Handles the logging in part of the service
pub mod login;
