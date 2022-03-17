/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   16 Mar 2022, 18:00:42
 * Last edited:
 *   17 Mar 2022, 09:36:39
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Library module for the todo-client.
**/

/// Enables the use of the lazy_static! macro
#[macro_use] extern crate lazy_static;

/// Collects all errors for this module
pub mod errors;
/// Implements the CLI and config parsing.
pub mod cli;
/// Implements the terminal UI.
pub mod tui;
