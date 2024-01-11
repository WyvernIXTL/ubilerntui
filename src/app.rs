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


use rand::thread_rng;
use rand::seq::SliceRandom;

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
      pub question_answer: QuestionAnswer,
      pub total_progress: usize,
      pub total_question_count: usize
}

impl App {
      pub fn new(question_answer: QuestionAnswer, total_progress: usize, total_question_count: usize) -> Self {
            Self { exit: false, item_list_state: ListState::default(), question_answer: question_answer, total_progress, total_question_count}
      }
}


#[derive(Default, Debug, Clone)]
pub struct QuestionAnswer {
      pub id: usize,
      pub question: String,
      pub possible_answers: Vec<String>,
      pub right_answer: usize,
      pub user_answer: Option<usize>,
      pub count_correctly_answered: usize,
}

impl QuestionAnswer {
      pub fn new<S: ToString>(id: usize, question: S, possible_answers: Vec<S>, right_answer: usize) -> Self {
            Self {
                  id: id,
                  question: question.to_string(), 
                  possible_answers: possible_answers.iter().map(|s| s.to_string()).collect(), 
                  right_answer: right_answer, 
                  user_answer: None,
                  count_correctly_answered: 0,
            }
      }

      pub fn scramble(&mut self) {
            let mut nums: Vec<usize> = (0..4).collect();
            nums.shuffle(&mut thread_rng());

            for i in 0..self.possible_answers.len() {
                  if i == self.right_answer {
                        self.right_answer = nums[i];
                  }
                  self.possible_answers.swap(i, nums[i]);
            }
      }
}