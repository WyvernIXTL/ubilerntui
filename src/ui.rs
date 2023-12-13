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
            Borders,
            List,
            ListItem,
            ListState,
            HighlightSpacing
      },
      style::Color,
      layout::Alignment
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

use crate::app::{App, QuestionAnswer};


pub fn draw(frame: &mut Frame, app: &mut App) {
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

      if app.question_answer.user_answer.is_none() {
            let mut bottom_help_bar_text = vec!["(q)/(esc) quit", "(w) go up", "(s) go down", "(e) select"];
            render_bottom_help_bar(frame, chunks[2], &mut bottom_help_bar_text);
      } else {
            let mut bottom_help_bar_text = vec!["(q)/(esc) quit", "(e) try again"];
            render_bottom_help_bar(frame, chunks[2], &mut bottom_help_bar_text);
      }

      render_selector_list(frame, chunks[1], &app.question_answer, &mut app.item_list_state);
}


pub fn render_selector_list(frame: &mut Frame, area: Rect, q: &QuestionAnswer, item_list_state: &mut ListState) {
      let style_correct = Style::default().fg(Color::Black).bg(Color::Green);
      let style_wrong = Style::default().fg(Color::Black).bg(Color::Red);

      let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                  Constraint::Length(2),
                  Constraint::Min(1),
            ])
            .split(area);

      let question = Paragraph::new("  ".to_owned() + q.question.as_str())
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::TOP));

      frame.render_widget(question, chunks[0]);


      let mut list_items = Vec::<ListItem>::new();
      if let Some(user_answer) = q.user_answer {
            for (i, e) in q.possible_answers.iter().enumerate() {
                  if i == q.right_answer {
                        list_items.push(ListItem::new(e.as_str()).style(style_correct));
                  } else if i == user_answer {
                        list_items.push(ListItem::new(e.as_str()).style(style_wrong));
                  } else {
                        list_items.push(ListItem::new(e.as_str()));
                  }
            }
      } else {
          list_items = q.possible_answers.iter().map(|s| ListItem::new(s.as_str())).collect();
      }

      let selector_list = List::new(list_items)
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM))
            .style(Style::default())
            .highlight_style(Style::default().fg(Color::Black).bg(Color::LightYellow))
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">>");

      frame.render_stateful_widget(selector_list, chunks[1], item_list_state);
}


pub fn render_bottom_help_bar(frame: &mut Frame, area: Rect, text: &mut Vec<&str>) {
      if text.len() == 1 {
            text.resize(3, "");
            text.swap(0, 1);
      } else if text.len() == 0 {
            text.resize(2, "");
      }

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
            ).block(block_right_open.clone()).alignment(Alignment::Center), 
            chunks[0]
      );

      for i in 1..text.len()-1 {
            frame.render_widget(
                  Paragraph::new(
                        Span::styled(text[i], Style::default())
                  ).block(block_left_right_open.clone()).alignment(Alignment::Center), 
                  chunks[i]
            );
      }

      frame.render_widget(
            Paragraph::new(
                  Span::styled(*text.last().unwrap(), Style::default())
            ).block(block_left_open.clone()).alignment(Alignment::Center), 
            *chunks.last().unwrap()
      );
}