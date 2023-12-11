/**
 * ratatui-selector
 * Copyright (C) 2023 Adam McKellar
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


use tracing::{
      debug, 
      error, 
      info, 
      warn, 
      trace, 
      span, 
      Level, 
      instrument, 
      debug_span,
      error_span,
      trace_span
};
use color_eyre::{
      Section, 
      eyre::{
            Report,
            Result,
            WrapErr,
            bail
      }
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub mod logging;
use logging::start_tracing;

pub mod tui;
use tui::Tui;

pub mod hooks;
use hooks::eyre_term_exit_hook;

pub mod app;
use app::App;

pub mod event;

pub mod update;

pub mod ui;


const APPLICATION_DIR_NAME: &str = "ratatui-selector";
const LOG_DIR_NAME: &str = "logs";


fn main() -> Result<()> {
      let entered_alternative_mode = Arc::new(AtomicBool::new(false));
      eyre_term_exit_hook(entered_alternative_mode.clone())?;

      let _tracing_guard = start_tracing(LOG_DIR_NAME, APPLICATION_DIR_NAME)?;


      entered_alternative_mode.swap(true, Ordering::Relaxed);
      let mut term = Tui::new_with_term()?;
      term.enter()?;
      trace!(id = "MSG-0001", "Entered alternative screen mode.");


      let mut app = App::new();
      let event_handler = event::InputEventHandler::new(120);

      let main_span = trace_span!("Main Loop", id = "MSG-0002").entered();
      loop {
            while let Ok(event) = event_handler.receiver.try_recv() {
                  update::update(event, &mut app)?;
            }
            if app.get_exit() {
                  break;
            }
            term.draw(app)?;
      }
      main_span.exit();


      term.exit()?;
      Ok(())
}
