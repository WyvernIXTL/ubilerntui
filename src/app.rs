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



#[derive(Default, Debug, Clone)]
pub struct App {
      pub exit: bool,
      pub item_list_state: ListState,
      pub question_answer: QuestionAnswer
}

impl App {
      pub fn new(question_answer: QuestionAnswer) -> Self {
            Self { exit: false, item_list_state: ListState::default(), question_answer: question_answer}
      }
}


#[derive(Default, Debug, Clone)]
pub struct QuestionAnswer {
      pub question: String,
      pub possible_answers: Vec<String>,
      pub right_answer: usize,
      pub user_answer: Option<usize>
}

impl QuestionAnswer {
      pub fn new<S: ToString>(question: S, possible_answers: Vec<S>, right_answer: usize) -> Self {
            Self { question: question.to_string(), 
                  possible_answers: possible_answers.iter().map(|s| s.to_string()).collect(), 
                  right_answer: right_answer, 
                  user_answer: None
            }
      }
}