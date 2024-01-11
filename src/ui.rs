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
            HighlightSpacing,
            LineGauge,
            Padding,
      },
      style::Color,
      layout::Alignment,
      symbols,
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
                  Constraint::Length(1),
                  Constraint::Length(1),
                  Constraint::Min(1),
                  Constraint::Length(3),
            ])
            .split(area);
            
      let title = Paragraph::new(
            Text::styled(
                  "UBI Lern TUI", 
                  Style::default()
            )
      ).block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2))
      );

      frame.render_widget(title, chunks[0]);

      render_total_progress(frame, chunks[1], app.total_progress, app.total_question_count);

      render_question_progress(frame, chunks[2], &app.question_answer);

      render_selector_list(frame, chunks[3], &app.question_answer, &mut app.item_list_state);

      if app.question_answer.user_answer.is_none() {
            let mut bottom_help_bar_text = vec!["(q)/(esc) Beenden", "(w) Hoch", "(s) Runter", "(e) Auswählen"];
            render_bottom_help_bar(frame, chunks[4], &mut bottom_help_bar_text);
      } else {
            let mut bottom_help_bar_text = vec!["(q)/(esc) Beenden", "(e) Nächste Frage"];
            render_bottom_help_bar(frame, chunks[4], &mut bottom_help_bar_text);
      }

}

fn render_total_progress(frame: &mut Frame, area: Rect, prog: usize, total: usize) {
      debug_assert!(prog <= total);
      let ratio = prog as f64 / total as f64;

      let progress_bar = LineGauge::default()
            .block(Block::default().borders(Borders::NONE).padding(Padding::horizontal(3)))
            .label("Fortschritt")
            .ratio(ratio)
            .gauge_style(Style::new().fg(Color::Green))
            .line_set(symbols::line::THICK);

      frame.render_widget(progress_bar, area);
}

fn render_question_progress(frame: &mut Frame, area: Rect, q: &QuestionAnswer) {
      let progress: f64;
      let fg_color;
      if q.count_correctly_answered >= 3 {
            progress = 1.0;
            fg_color = Color::Green;
      } else {
            progress = (q.count_correctly_answered + 1) as f64 * 0.25;
            if q.count_correctly_answered == 2 {
                  fg_color = Color::Yellow;
            } else if q.count_correctly_answered == 1 {
                  fg_color = Color::Yellow;
            } else {
                fg_color = Color::Red;
            }
      }

      let progress_bar = LineGauge::default()
            .block(Block::default().borders(Borders::NONE).padding(Padding::horizontal(3)))
            .label("Fragen-Fortschritt")
            .ratio(progress)
            .gauge_style(Style::new().fg(fg_color))
            .line_set(symbols::line::THICK);

      frame.render_widget(progress_bar, area);
}


fn render_selector_list(frame: &mut Frame, area: Rect, q: &QuestionAnswer, item_list_state: &mut ListState) {
      let style_correct = Style::default().fg(Color::Black).bg(Color::Green);
      let style_wrong = Style::default().fg(Color::Black).bg(Color::Red);

      let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                  Constraint::Length(2),
                  Constraint::Min(1),
            ])
            .split(area);

      let question = Paragraph::new(q.question.as_str())
            .block(Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2))
      );

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
            .block(Block::default()
                  .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                  .border_type(BorderType::Rounded)
            )
            .style(Style::default())
            .highlight_style(Style::default().fg(Color::Black).bg(Color::LightYellow))
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">>");

      frame.render_stateful_widget(selector_list, chunks[1], item_list_state);
}


fn render_bottom_help_bar(frame: &mut Frame, area: Rect, text: &mut Vec<&str>) {
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