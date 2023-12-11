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
            Stylize,
            Layout,
            Direction,
            Constraint,
            Text,
            Style,
            Span,
            Rect
      },
      widgets::{
            Paragraph,
            Block,
            BorderType,
            Borders
      }
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


      let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
      ])
      .split(area);


      let border_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
            
      let title = Paragraph::new(
            Text::styled(
                  "Question and Answer List", 
                  Style::default()
            )
      ).block(border_block.clone());

      frame.render_widget(title, chunks[0]);


      let bottom_help_bar_text = vec!["(q)/(esc) quit", "(w) go up", "(s) go down", "(e) select"];

      render_bottom_help_bar(frame, chunks[2], bottom_help_bar_text);

      
}


pub fn render_bottom_help_bar(frame: &mut Frame, area: Rect, text: Vec<&str>) {
      let count = u32::try_from(text.len()).unwrap();
      let constraint_single = Constraint::Ratio(1, count);

      let mut constraints: Vec<Constraint> = Vec::new();
      for _i in 0..count {
            constraints.push(constraint_single.clone());
      }

      let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

      let block_right_open = Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM);

      let block_left_open = Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM);

      let block_left_right_open = Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::TOP | Borders::BOTTOM);

      frame.render_widget(
            Paragraph::new(
                  Span::styled(text[0], Style::default())
            ).block(block_right_open.clone()), 
            chunks[0]
      );

      for i in 1..text.len()-1 {
            frame.render_widget(
                  Paragraph::new(
                        Span::styled(text[i], Style::default())
                  ).block(block_left_right_open.clone()), 
                  chunks[i]
            );
      }

      frame.render_widget(
            Paragraph::new(
                  Span::styled(text[3], Style::default())
            ).block(block_left_open.clone()), 
            chunks[3]
      );
}