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


use color_eyre::eyre::Result;

use ratatui::widgets::ListState;

use crossterm::event::KeyCode::{self, Char};

use crate::app::App;
use crate::event::EventType;



pub fn update(event: EventType, app: &mut App) -> Result<()> {
      match event {
            EventType::Resize(w, h) => {},
            EventType::Mouse(mouse_event) => {},
            EventType::Key(key_event) => match app.question_answer.user_answer {
                  Some(i) => match key_event.code {
                        Char('q') | KeyCode::Esc => app.exit = true,
                        Char('e') | KeyCode::Enter => {app.question_answer.user_answer = None; app.item_list_state.select(None)},
                        _ => {}
                  },
                  None => match key_event.code {
                        Char('q') | KeyCode::Esc => app.exit = true,
                        Char('w') | KeyCode::Up => list_move_up(&mut app.item_list_state),
                        Char('s') | KeyCode::Down => list_move_down(&mut app.item_list_state, app.question_answer.possible_answers.len()),
                        Char('e') | KeyCode::Enter => if let Some(i) = app.item_list_state.selected() {
                              if i < app.question_answer.possible_answers.len(){
                                    app.question_answer.user_answer = Some(i);
                                    app.item_list_state.select(None);

                                    if app.question_answer.right_answer == i {
                                          app.question_answer.count_correctly_answered += 1;
                                    } else {
                                          app.question_answer.count_correctly_answered = 0;
                                    }
                              }
                        },
                        _ => {}
                  }
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