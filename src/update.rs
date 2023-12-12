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

use ratatui::widgets::ListState;

use crossterm::event::KeyCode::{self, Char};

use crate::app::App;
use crate::event::EventType;



pub fn update(event: EventType, app: &mut App) -> Result<()> {
      match event {
            EventType::Resize(w, h) => {},
            EventType::Mouse(mouse_event) => {},
            EventType::Key(key_event) => match key_event.code {
                  Char('q') | KeyCode::Esc => app.exit = true,
                  Char('w') => list_move_up(&mut app.item_list_state),
                  Char('s') => list_move_down(&mut app.item_list_state, app.item_list.len()),
                  _ => {}
            }
      }

      Ok(())
}


fn list_move_up(list: &mut ListState) {
      let current = list.selected().unwrap_or(0);
      let next;
      if current == 0 {
            next = 0;
      } else {
          next = current-1;
      }
      list.select(Some(next));
}

fn list_move_down(list: &mut ListState, list_size: usize) {
      let current = list.selected().unwrap_or(list_size-1);
      let next;
      if current < list_size-1 {
            next = current+1;
      } else {
          next = current;
      }      
      list.select(Some(next));
}