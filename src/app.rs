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


use rand::{thread_rng, rngs::ThreadRng, RngCore};
use rand::seq::SliceRandom;

use ratatui::widgets::ListState;


/// This struct contains the programs state.
/// 
/// It's purpose is to serve as state, which will then be updated by [update()](crate::update::update).
/// It contains a single [QuestionAnswer], the state of the [List](ratatui::widgets::List) displayed,
/// the signal for exit and the total progress[^note].
/// 
/// [^note]: As else this progress count would need to be querried in the [db](crate::db::DB::get_total_progress), every frame.
/// 
/// ```
/// let first_question = QuestionAnswer::new(0, "What is 1+1?", vec!["3", "2", "1", "4"], 1);
/// let mut app = App::new(
///       first_question, 
///       0, 
///       3
/// );
/// ```
/// 
#[derive(Default, Debug, Clone)]
pub struct App {
      pub exit: bool,
      pub item_list_state: ListState,
      pub question_answer: QuestionAnswer,
      pub total_progress: usize,
      pub total_question_count: usize,
      pub rng: ThreadRng
}

impl App {
      /// Returns [App] struct.
      /// 
      /// Takes `total_progress` which is the sum of all correct trys of the user.
      /// Takes `total_question_count` which is the count of questions * 3.
      pub fn new(question_answer: QuestionAnswer, total_progress: usize, total_question_count: usize) -> Self {
            Self { exit: false, item_list_state: ListState::default(), question_answer: question_answer, total_progress, total_question_count, rng: thread_rng() }
      }
}


/// This struct saves a question, the right answer and wrong answers.
/// 
/// It also contains functions for [scrambling](QuestionAnswer::scramble) the answers, but with keeping track of the right answer.
/// It also holds the users input used for rendering the result to the user.
/// 
/// ```
/// let first_question = QuestionAnswer::new(0, "What is 1+1?", vec!["3", "2", "1", "4"], 1);
/// ```
/// 
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

      /// Scramble right and wrong answers.
      /// 
      /// `right_answer` always points at the index with the right answer in `possible_answers`.
      pub fn scramble<R: RngCore>(&mut self, rng: &mut R) {
            let mut index_vec: Vec<usize> = (0..self.possible_answers.len()).collect();
            index_vec.shuffle(rng);

            self.possible_answers = (0..self.possible_answers.len())
                  .map(|i| self.possible_answers[index_vec[i]].clone())
                  .collect();

            self.right_answer = index_vec.iter().position(|&i| i == self.right_answer).unwrap();
      }
}


#[cfg(test)]
mod tests {
      use super::*;
      use pretty_assertions::assert_eq;

      #[test]
      fn test_scramble() {
            let mut q = QuestionAnswer::new(0, "nan", vec!["0", "1", "2", "3"], 0);
            let mut rng = thread_rng();
            for _ in 0..10 {
                  q.scramble(&mut rng);
                  assert_eq!("0", q.possible_answers[q.right_answer]);
            }
            assert!(q.possible_answers.iter().any(|s| s=="1"));
            assert!(q.possible_answers.iter().any(|s| s=="2"));
            assert!(q.possible_answers.iter().any(|s| s=="3"));
      }

      #[test]
      fn test_scramble_with_longer_vectors() {
            let mut q = QuestionAnswer::new(0, "nan", vec!["0", "1", "2", "3", "4", "5"], 0);
            let mut rng = thread_rng();

            q.scramble(&mut rng);
            
            assert_eq!("0", q.possible_answers[q.right_answer]);
            assert!(q.possible_answers.iter().any(|s| s=="1"));
            assert!(q.possible_answers.iter().any(|s| s=="2"));
            assert!(q.possible_answers.iter().any(|s| s=="3"));
            assert!(q.possible_answers.iter().any(|s| s=="4"));
            assert!(q.possible_answers.iter().any(|s| s=="5"));
      }
}