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


use std::fs::read;
use std::path::PathBuf;

use color_eyre::eyre::Result;
use pdf_extract::extract_text_from_mem;
use regex::Regex;
use once_cell::sync::Lazy;


pub fn read_pdf_to_string(path: PathBuf) -> Result<String> {
      let bytes = read(path)?;
      Ok( extract_text_from_mem(&bytes)? )
}

enum ParseState {
      Search,
      QuestionStartFound,
      QuestionEndFound
}

enum AnswerState {
      Begin,
      Middle
}


pub fn parse_pdf(s: String) -> Result<Vec<(usize, String, String, Vec<String>)>> {
      let mut result = vec![];

      let mut state = ParseState::Search;
      let mut answer_state = AnswerState::Begin;
      let mut answer_a = true;

      let mut partial_string: String = "".to_owned();
      let mut id: usize = 0;
      let mut question: String = "".to_owned();
      let mut answer: String = "".to_owned();
      let mut wrong_answers: Vec<String> = vec![];

      for l in s.lines() {
            let l = l.trim();
            match state {
                  ParseState::Search => {
                        if l.len() != 0 {
                              if let Some((id_start, q_start)) = valid_start_question(l) {
                                    if let Some((id_end, _)) = valid_end_question(l) {
                                          if id_start == id_end {
                                                if let Some(q) = extract_sandwiched_question(l) {
                                                      id = id_start;
                                                      question = q;
                                                      state = ParseState::QuestionEndFound;
                                                }
                                          }
                                    } else {
                                          id = id_start;
                                          question = q_start;
                                          state = ParseState::QuestionStartFound;
                                    }
                              }
                        }
                  },
                  ParseState::QuestionStartFound => {
                        if l.len() != 0 {
                              if let Some((id_end, q_end)) = valid_end_question(l) {
                                    if id == id_end {
                                          question.push_str(" ");
                                          question.push_str(&q_end);
                                          state = ParseState::QuestionEndFound;
                                          answer_state = AnswerState::Begin;
                                    } else {
                                          state = ParseState::Search;
                                    }
                              } else {
                                    question.push_str(" ");
                                    question.push_str(l);
                              }
                        } else {
                              state = ParseState::Search;
                        }
                  },
                  ParseState::QuestionEndFound => {
                        match answer_state {
                              AnswerState::Begin => {
                                    if l.len() != 0 {
                                          partial_string = "".to_owned();
                                          if let Some((num, answer_start)) = extract_answer_number_and_answer_start(l) {
                                                partial_string = answer_start;
                                                answer_state = AnswerState::Middle;
                                                answer_a = &num == "a";
                                          }
                                    }
                              },
                              AnswerState::Middle => {
                                    if l.len() != 0 {
                                          partial_string.push_str(" ");
                                          partial_string.push_str(l);
                                    } else {
                                          answer_state = AnswerState::Begin;
                                          if answer_a {
                                                answer = partial_string.clone();
                                          } else {
                                                wrong_answers.push(partial_string.clone());
                                                if wrong_answers.len() >= 3 {
                                                      state = ParseState::Search;
                                                      result.push((id, question.clone(), answer.clone(), wrong_answers.clone()));
                                                      wrong_answers.clear();
                                                }
                                          }
                                    }
                              }
                        }
                  }
            }
            
      }


      Ok( result )
}


fn valid_start_question(l: &str) -> Option<(usize, String)> {
      static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?P<id>[0-9]{1,3})\.\s+(?P<question>.*)$").unwrap());
      if let Some(caps) = REG.captures(l) {
            Some((
                  usize::from_str_radix(&caps["id"], 10).unwrap(),
                  String::from(&caps["question"])
            ))
      } else {
            None
      }
}

fn valid_end_question(l: &str) -> Option<(usize, String)> {
      static  REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*(?P<question>.*)\s+\[(?P<id>[0-9]{1,3})\]\s*$").unwrap());
      if let Some(caps) = REG.captures(l) {
            let q: &str = &caps["question"];
            Some((
                  usize::from_str_radix(&caps["id"], 10).unwrap(),
                  String::from(q.trim())
            ))
      } else {
            None
      }
}

fn extract_sandwiched_question(l: &str) -> Option<String> {
      static  REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]{1,3}\.\s+(?P<question>.+)\s+\[[0-9]{1,3}\]\s*$").unwrap());
      if let Some(caps) = REG.captures(l) {
            Some(String::from(&caps["question"]))
      } else {
            None
      }
}

fn extract_answer_number_and_answer_start(l: &str) -> Option<(String, String)> {
      static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?P<number>[abcd])\)\s+(?P<answer>.*)$").unwrap());
      if let Some(caps) = REG.captures(l) {
            Some((
                  String::from(&caps["number"]), 
                  String::from(&caps["answer"])
            ))
      } else {
            None
      }
}



#[cfg(test)]
mod tests {
      use super::*;

      #[test]
      fn test_valid_start_question() {
            let good_string = "123. something";
            let bad_string_no_number = "abc. something";
            let bad_string_no_dot = "123 something";
            let bad_string_too_many_numbers = "1234. something";

            assert_eq!(valid_start_question(good_string), Some((123usize, "something".to_owned())));
            assert_eq!(valid_start_question(bad_string_no_number), None);
            assert_eq!(valid_start_question(bad_string_no_dot), None);
            assert_eq!(valid_start_question(bad_string_too_many_numbers), None);
      }

      #[test]
      fn test_valid_end_question() {
            let good_string = "something [123]";
            let good_string_trailing_white_space = "something [123]   ";
            let good_string_many_white_space = "       some thing      [123]   ";
            let bad_string_missing_bracket_left = "something 123]";
            let bad_string_missing_bracket_right = "something [123";
            let bad_string_not_eofl = "something [123]   something";
            let bad_string_id_not_numeric = "something [123 ]";

            assert_eq!(valid_end_question(good_string), Some((123usize, "something".to_owned())));
            assert_eq!(valid_end_question(good_string_trailing_white_space), Some((123usize, "something".to_owned())));
            assert_eq!(valid_end_question(good_string_many_white_space), Some((123usize, "some thing".to_owned())));
            assert_eq!(valid_end_question(bad_string_missing_bracket_left), None);
            assert_eq!(valid_end_question(bad_string_missing_bracket_right), None);
            assert_eq!(valid_end_question(bad_string_not_eofl), None);
            assert_eq!(valid_end_question(bad_string_id_not_numeric), None);
      }

      #[test]
      fn test_extract_sandwiched_question() {
            let good_string = "123. something [456]";
            let bad_string_no_white_space = "123.something [123]";
            let bad_string_no_white_space_2 = "123. something[123]";
            let bad_string_only_white_space = "123.  [123]";
            let bad_string_bad_number = "a123. something [456]";

            assert_eq!(extract_sandwiched_question(good_string), Some("something".to_owned()));
            assert_eq!(extract_sandwiched_question(bad_string_bad_number), None);
            assert_eq!(extract_sandwiched_question(bad_string_no_white_space), None);
            assert_eq!(extract_sandwiched_question(bad_string_no_white_space_2), None);
            assert_eq!(extract_sandwiched_question(bad_string_only_white_space), None);
      }

      #[test]
      fn test_extract_answer_number_and_answer_start() {
            let good_string = "a) something";
            let bad_string_number_missing = ") something";
            let bad_string_no_space = "a)something";

            assert_eq!(extract_answer_number_and_answer_start(good_string), Some(("a".to_owned(), "something".to_owned())));
            assert_eq!(extract_answer_number_and_answer_start(bad_string_number_missing), None);
            assert_eq!(extract_answer_number_and_answer_start(bad_string_no_space), None);
      }

      #[test]
      fn test_parse_clusterfuck_pdf() -> Result<()> {
            let test_raw_string = "

128.   q part1
q part2   [128]

a)   correct answer

b)   wrong answer 1

c)   wrong answer 2

d)   wrong answer 3




129.   q part1
q part2   [129]

a)   correct answer

b)   wrong answer 1

c)   wrong answer 2

d)   wrong answer 3





      something something (ABC) / STD: 02/42

89

something something – something –
something something something


127.   q part1
q part2   [127]

a)   correct answer part 1
correct answer part 2

b)   wrong answer 1 part 1
wrong answer 1 part 2
wrong answer 1 part 3

c)   wrong answer 2 part 1
wrong answer 2 part 2

d)   wrong answer 3 part 1
wrong answer 3 part 2

            ";

            let expected = vec![
                  (128usize, "q part1 q part2".to_owned(), "correct answer".to_owned(), vec!["wrong answer 1".to_owned(), "wrong answer 2".to_owned(), "wrong answer 3".to_owned()]),
                  (129, "q part1 q part2".to_owned(), "correct answer".to_owned(), vec!["wrong answer 1".to_owned(), "wrong answer 2".to_owned(), "wrong answer 3".to_owned()]),
                  (
                        127, 
                        "q part1 q part2".to_owned(), 
                        "correct answer part 1 correct answer part 2".to_owned(), 
                        vec![
                              "wrong answer 1 part 1 wrong answer 1 part 2 wrong answer 1 part 3".to_owned(), 
                              "wrong answer 2 part 1 wrong answer 2 part 2".to_owned(), 
                              "wrong answer 3 part 1 wrong answer 3 part 2".to_owned()
                              ]
                  )
            ];

            let res = parse_pdf(test_raw_string.to_owned())?;

            assert_eq!(res, expected);

            Ok(())
      }
}