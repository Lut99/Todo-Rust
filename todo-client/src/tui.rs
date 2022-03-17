/* TUI.rs
 *   by Lut99
 *
 * Created:
 *   17 Mar 2022, 09:35:54
 * Last edited:
 *   17 Mar 2022, 14:59:26
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the todo-client tool's Terminal UI (TUI).
**/

use std::io::{self, Stdout};

use crossterm::execute;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders};

pub use crate::errors::TuiError as Error;


/***** LIBRARY STRUCTS *****/
/// The interface to the backend TUI library. You'll probably only ever need one of these.
pub struct TerminalUi {
    /// The terminal backend used for the TUI.
    terminal : Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalUi {
    /// Constructor for the TerminalUi.
    /// 
    /// **Returns**  
    /// The new TerminalUi instance on success, or a TuiError otherwise.
    pub fn new() -> Result<Self, Error> {
        // Enter raw mode
        if let Err(err) = enable_raw_mode() {
            return Err(Error::RawModeEnableError{ err });
        }

        // Create a stdout handle
        let mut stdout = io::stdout();
        
        // Put it in the correct settings
        if let Err(err) = execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
            return Err(Error::ExecuteError{ err });
        }

        // Create the TUI backend
        let backend = CrosstermBackend::new(stdout);
        let terminal = match Terminal::new(backend) {
            Ok(terminal) => terminal,
            Err(err)     => { return Err(Error::TerminalCreateError{ err }); }
        };

        // Create a self instance with that backend
        Ok(Self {
            terminal,
        })
    }



    /// Builds the user interface for the TerminalUi.
    /// 
    /// **Arguments**
    ///  * ``
    /// 
    /// **Returns**  
    /// Nothing on success, or else a TuiError.
    pub fn render_ui(&mut self) -> Result<(), Error> {
        // Draw on the internal terminal
        match self.terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Hello, world!")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        }) {
            Ok(_)    => Ok(()),
            Err(err) => Err(Error::TerminalDrawError{ err }),
        }
    }
}

impl Drop for TerminalUi {
    fn drop(&mut self) {
        // Leave raw mode
        if let Err(err) = disable_raw_mode() {
            panic!("{}", Error::RawModeDisableError{ err });
        }

        // Disable the terminal mode
        if let Err(err) = execute!(self.terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture) {
            panic!("{}", Error::ExecuteError{ err });
        }

        // Finally, show the cursor again
        if let Err(err) = self.terminal.show_cursor() {
            panic!("{}", Error::ShowCursorError{ err });
        }
    }
}

impl Default for TerminalUi {
    /// Default constructor for the TerminalUi.
    #[inline]
    fn default() -> Self {
        match Self::new() {
            Ok(res)  => res,
            Err(err) => { panic!("Could not instantiate default TerminalUi: {}", err); }
        }
    }
}
