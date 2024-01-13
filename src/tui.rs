/**
 * ubilerntui
 * Copyright (C) 2024 Adam McKellar
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */


use crossterm::{
      event::{
            DisableMouseCapture, 
            EnableMouseCapture
      },
      terminal::{
            self,
            EnterAlternateScreen, 
            LeaveAlternateScreen
      }
};
use ratatui::prelude::{
            CrosstermBackend, 
            Terminal
      };
use std::io::{
      self, 
      Stderr
};
use color_eyre::{
      Section, 
      eyre::{
            Result,
            WrapErr
      }
};

use crate::ui;
use crate::app;

/// Holds handle to functionality of underlying terminal.
/// 
/// There are functions for entering the alternative mode of the terminal 
/// and exiting it.
/// ```
/// let tui = Tui::new_with_term()?;
/// tui.enter()?;
/// /// alternative mode
/// tui.exit()?
/// ```
pub struct Tui {
      terminal: Terminal<CrosstermBackend<Stderr>>
}

impl Tui {
      pub fn new(terminal: Terminal<CrosstermBackend<Stderr>>) -> Self {
            Self {terminal}
      }

      pub fn new_with_term() -> Result<Tui, io::Error> {
            Ok( Tui::new(Terminal::new(CrosstermBackend::new(io::stderr()))?) )
      }

      /// Enters alternative mode of terminal and hides cursor.
      /// ```
      /// let tui = Tui::new_with_term()?;
      /// tui.enter()?;
      /// ```
      pub fn enter(&mut self) -> Result<()> {
            terminal::enable_raw_mode()
                  .wrap_err("Failed enabling raw mode for terminal.")
                  .suggestion("Use another terminal, like Wezterm.")?;

            crossterm::execute!(
                  io::stderr(),
                  EnterAlternateScreen,
                  EnableMouseCapture
            )?;

            self.terminal.hide_cursor()
                  .wrap_err("Failed hiding cursor.")?;
            self.terminal.clear()
                  .wrap_err("Failed clearing terminal.")?;
            Ok(())
      }

      /// Exits alternative mode of terminal making terminal usable agains as such.
      /// ```
      /// let tui = Tui::new_with_term()?;
      /// tui.enter()?;
      /// tui.exit()?;
      /// ```
      pub fn exit(&mut self) -> Result<()> {
            partial_exit()?;

            self.terminal.show_cursor()
                  .wrap_err("Failed showing cursor.")?;

            Ok(())
      }

      /// Draws TUI to terminal.
      pub fn draw(&mut self, app: &mut app::App) -> Result<()> {
            self.terminal.draw(|frame| ui::draw(frame, app))?;
            Ok(())
      }

      pub fn get_size(self) -> Result<(u16, u16)> {
            let rect = self.terminal.size()?;
            Ok((rect.right()-rect.left(), rect.bottom()-rect.top()))
      }
}

/// Exits termial alternative mode and raw mode like exit without having access to [Tui] struct.
/// 
/// This function is needed for error handling and restoring the terminal to an usable state after a panic.
/// ```
/// partial_exit()?;
/// ```
pub fn partial_exit() -> Result<()> {
      crossterm::execute!(
            io::stderr(), 
            LeaveAlternateScreen,
            DisableMouseCapture
      )?;
      terminal::disable_raw_mode()
            .wrap_err("Failed disabling raw mode, your terminal is fucked")
            .suggestion("Use a terminal supported by crossterm.")?;

      Ok(())
}
