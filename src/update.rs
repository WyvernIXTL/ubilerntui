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


use color_eyre::{
      Section, 
      eyre::{
            Report,
            Result,
            WrapErr,
            bail
      }
};

use crossterm::event::KeyCode::{self, Char};

use crate::app::App;
use crate::event::EventType;



pub fn update(event: EventType, app: &mut App) -> Result<()> {
      match event {
            EventType::Resize(w, h) => {},
            EventType::Mouse(mouse_event) => {},
            EventType::Key(key_event) => match key_event.code {
                  Char('q') | KeyCode::Esc => app.set_exit_true(),
                  _ => {}
            }
            _ => {}
      }

      Ok(())
}