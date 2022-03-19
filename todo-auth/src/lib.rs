/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 11:47:45
 * Last edited:
 *   19 Mar 2022, 15:36:42
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the library entrypoint for the todo-auth application.
**/

/// Allows the use of the log crate macros
extern crate log;

/// Collects all errors for this package.
pub mod errors;
/// Contains specifications for the todo-auth service
pub mod spec;
/// Handles the logging in part of the service
pub mod login;
