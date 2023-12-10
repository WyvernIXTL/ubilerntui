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




use ratatui::{
      prelude::{
            Frame,
            Stylize
      },
      widgets::Paragraph
};
use color_eyre::{
      Section, 
      eyre::{
            self,
            Report,
            Result,
            WrapErr
      }
};

use crate::App;


pub fn draw(frame: &mut Frame, app: App) {
      let area = frame.size();
      frame.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                  .white()
                  .on_blue(),
            area,
      );
}