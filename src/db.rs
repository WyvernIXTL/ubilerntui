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
use rusqlite::Connection;


use crate::app::QuestionAnswer;
use crate::fs::get_local_dir;


const DB_NAME: &str = "ubilerndb.sqlite3";
const TOTAL_COUNT_TRYS_PER_QUESTION: usize = 3;

const SQL_CREATE_QUESTION_TABLE: &str = "CREATE TABLE IF NOT EXISTS questions (
      id                            INTEGER PRIMARY KEY,
      question                      TEXT NOT NULL,     
      answers_0                     TEXT NOT NULL,
      answers_1                     TEXT NOT NULL,
      answers_2                     TEXT NOT NULL,
      answers_3                     TEXT NOT NULL,
      correctly_answered            INTEGER NOT NULL
)";


/// Struct holding the [Connection] with an sqlite database.
/// 
/// Moreover there are many helper fuctions for manipulating the questions state: 
/// - Inserting questions.
/// - Getting [QuestionAnswer] structs at random.
/// - Clearing questions.
/// - Clearing progress.
/// 
/// ```
/// let db = DB::new("db")?;
/// ```
/// 
/// By initializing `DB` via `new` a file is created 
/// in the program specifique local folder of your os
/// if it does not allready exist.
/// 
/// Only one [DB] struct should exist.
pub struct DB {
      pub db: Connection
}

impl DB {
      /// Connects to database in local folder of program[^note].
      /// [^note]: via [get_local_dir()]
      /// 
      /// If the database or folder does not exist, the database and folder are created.
      /// The table `questions` is also created holding all necessary information per question.
      pub fn new(db_dir_name: &str) -> Result<Self> {
            let db_path = get_local_dir(db_dir_name)?.join(DB_NAME);
            let db = Connection::open(db_path)?;
            db.execute(
                  SQL_CREATE_QUESTION_TABLE,
                  ()
            )?;
            Ok(Self { db: db })
      }

      /// Inserts question into database (table `question`).
      /// ```
      /// let db = DB::new("db")?;
      /// db.insert(1, "What is 1+1 ?", "2", vec!["1", "0", "3"])?;
      /// ```
      pub fn insert<S: ToString>(&self, id: usize, question: S, right_answer: S, false_answers: Vec<S>) -> Result<()> {
            debug_assert!(false_answers.len() == TOTAL_COUNT_TRYS_PER_QUESTION);

            self.db.execute(
                  "INSERT INTO questions (id, question, answers_0, answers_1, answers_2, answers_3, correctly_answered)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                  (&id, &question.to_string(), &right_answer.to_string(), &false_answers[0].to_string(), &false_answers[1].to_string(), &false_answers[2].to_string(), 0)
            )?;

            Ok(())
      }

      /// Inserts question as tuple into database (table `question`).
      /// ```
      /// let db = DB::new("db")?;
      /// db.insert( (1, "What is 1+1 ?", "2", vec!["1", "0", "3"]) )?;
      /// ```
      pub fn insert_tuple<S: ToString>(&self, (id, question, right_answer, false_answers): (usize, S, S, Vec<S>)) -> Result<()> {
            self.insert(id, question, right_answer, false_answers)
      } 

      /// Returns random question as [QuestionAnswer] where question is not answered consecutively correct more than 2 times.
      /// ```
      /// let db = DB::new("db")?;
      /// db.insert(1, "What is 1+1 ?", "2", vec!["1", "0", "3"])?;
      /// let q = db.get_random()?;
      /// ```
      pub fn get_random(&self) -> Result<QuestionAnswer> {
            Ok(
                  self.db.query_row(
                  "SELECT id, question, answers_0, answers_1, answers_2, answers_3, correctly_answered
                        FROM questions
                        WHERE correctly_answered < 3
                        ORDER BY RANDOM()
                        LIMIT 1"
                        , (), |f| {
                              let possible_answers = vec![f.get(2)?, f.get(3)?, f.get(4)?, f.get(5)?];
                              Ok(
                                    QuestionAnswer {
                                          id: f.get(0)?,
                                          question: f.get(1)?,
                                          possible_answers,
                                          right_answer: 0,
                                          user_answer: None,
                                          count_correctly_answered: f.get(6)?
                                    }
                              )
                        }
                  )?
            )
      }

      /// Update `question progress` of question with `id` with new value `new_count`.
      /// ```
      /// let db = DB::new("db")?;
      /// db.insert(1, "What is 1+1 ?", "2", vec!["1", "0", "3"])?;
      /// db.update_count_correct_answers(1, 2)?;
      /// ```
      pub fn update_count_correct_answers(&self, id: usize, new_count: usize) -> Result<()> {
            debug_assert!(new_count <= TOTAL_COUNT_TRYS_PER_QUESTION);
            self.db.execute(
                  "UPDATE questions
                  SET correctly_answered = ?1
                  WHERE id = ?2", 
                  (&new_count, &id)
            )?;

            Ok(())
      }

      /// Returns sum of the questions `question progress`.
      /// ```
      /// let db = DB::new("db")?;
      /// db.insert(1, "What is 1+1 ?", "2", vec!["1", "0", "3"])?;
      /// assert_eq!(db.get_total_progress()?, 0);
      /// db.update_count_correct_answers(1, 2)?;
      /// assert_eq!(db.get_total_progress()?, 2);
      /// 
      /// db.insert(2, "What is 1+2 ?", "3", vec!["1", "0", "2"])?;
      /// assert_eq!(db.get_total_progress()?, 2);
      /// db.update_count_correct_answers(2, 1)?;
      /// assert_eq!(db.get_total_progress()?, 3);
      /// ```
      pub fn get_total_progress(&self) -> Result<usize> {
            Ok(
                  self.db.query_row(
                        "SELECT sum(correctly_answered)
                        FROM questions", 
                        (),
                        |f| f.get(0)
                  )?
            )
      }

      /// Returns the amount/count of questions * the max count a question can be answered correct.
      /// 
      /// This amounts to the total work of the user 
      /// until he answered every question consecutively 3 times correct.
      /// ```
      /// let db = DB::new("db")?;
      /// assert_eq!(db.get_total_question_count()?, 0);
      /// 
      /// db.insert(1, "What is 1+1 ?", "2", vec!["1", "0", "3"])?;
      /// assert_eq!(db.get_total_question_count()?, 3);
      /// 
      /// db.insert(2, "What is 1+2 ?", "3", vec!["1", "0", "2"])?;
      /// assert_eq!(db.get_total_question_count()?, 6);
      /// ```
      pub fn get_total_question_count(&self) -> Result<usize> {
            let row_count: usize = self.db.query_row(
                  "SELECT count()
                  FROM questions", 
                  (), 
                  |f| f.get(0)
            )?;

            Ok(row_count * TOTAL_COUNT_TRYS_PER_QUESTION)
      }

      /// Checks if `questions` table has row entries.
      /// ```
      /// let db = DB::new("db")?;
      /// assert!(db.is_empty()?);
      /// 
      /// db.insert(1, "What is 1+1 ?", "2", vec!["1", "0", "3"])?;
      /// assert!(!db.is_empty()?);
      /// ```
      pub fn is_empty(&self) -> Result<bool> {
            let row_count: usize = self.db.query_row(
                  "SELECT count()
                  FROM questions", 
                  (), 
                  |f| f.get(0)
            )?;
            Ok(row_count == 0)
      }

      /// Clear all rows in `questions` table.
      /// ```
      /// let db = DB::new("db")?;
      /// db.insert(1, "What is 1+1 ?", "2", vec!["1", "0", "3"])?;
      /// assert!(!db.is_empty()?);
      /// 
      /// db.clear()?;
      /// assert!(db.is_empty()?);
      /// ```
      pub fn clear(&self) -> Result<()> {
            self.db.execute("DELETE FROM questions", ())?;
            Ok(())
      }

      /// Resets `question progress` of every question.
      /// ```
      /// let db = DB::new("db")?;
      /// db.insert(1, "What is 1+1 ?", "2", vec!["1", "0", "3"])?;
      /// db.update_count_correct_answers(1, 2)?;
      /// assert_eq!(db.get_total_progress()?, 2);
      /// 
      /// db.clear_progress()?;
      /// assert_eq!(db.get_total_progress()?, 0);
      /// ```
      pub fn clear_progress(&self) -> Result<()> {
            self.db.execute(
                  "UPDATE questions
                  SET correctly_answered = 0", 
                  ()
            )?;

            Ok(())
      }

      /// Checks if there are no questions with `question progress` less than 
      /// the max count a question can be answered correct.
      /// ```
      /// let db = DB::new("db")?;
      /// db.insert(1, "What is 1+1 ?", "2", vec!["1", "0", "3"])?;
      /// assert!(!db.no_open_questions()?);
      /// 
      /// db.update_count_correct_answers(1, 3)?;
      /// assert!(db.no_open_questions()?);
      /// ```
      pub fn no_open_questions(&self) -> Result<bool> {
            let row_count: usize = self.db.query_row(
                  "SELECT count()
                  FROM questions
                  WHERE correctly_answered < 3", 
                  (), 
                  |f| f.get(0)
            )?;
            Ok(row_count == 0)
      }
}



#[cfg(test)]
mod tests {
      use super::*;

      impl DB {
            /// Returns database, which works in memory. This is for testing purposes.
            fn new_in_memory() -> Result<Self> {
                  let db = Connection::open_in_memory()?;
                  db.execute(
                        SQL_CREATE_QUESTION_TABLE,
                        ()
                  )?;
                  Ok(Self { db: db })
            }
      }

      #[test]
      fn test_insertion_and_read_single() -> Result<()> {
            let db = DB::new_in_memory()?;
            let right_answer = "0";
            let false_answers = vec!["1", "2", "3"];
            
            db.insert(1, "nan", right_answer, false_answers.clone())?;

            let q = db.get_random()?;

            assert_eq!(q.id, 1);
            assert_eq!(q.right_answer, 0);
            assert_eq!(q.possible_answers[0], "0");
            assert_eq!(q.possible_answers[1..4], false_answers);
            assert_eq!(q.question, "nan");

            Ok(())
      }

      #[test]
      fn test_update_count_correct_answers() -> Result<()> {
            let db = DB::new_in_memory()?;
            let right_answer = "0";
            let false_answers = vec!["1", "2", "3"];
            db.insert(1, "nan", right_answer, false_answers.clone())?;

            let q = db.get_random()?;
            assert_eq!(q.count_correctly_answered, 0);

            db.update_count_correct_answers(1, 2)?;

            let q = db.get_random()?;
            assert_eq!(q.count_correctly_answered, 2);

            Ok(())
      }

      #[test]
      fn test_get_total_progress() -> Result<()> {
            let db = DB::new_in_memory()?;
            let right_answer = "0";
            let false_answers = vec!["1", "2", "3"];
            db.insert(1, "nan", right_answer, false_answers.clone())?;

            db.update_count_correct_answers(1, 2)?;

            assert_eq!(db.get_total_progress()?, 2);

            db.insert(2, "nan", right_answer, false_answers.clone())?;
            db.update_count_correct_answers(2, 3)?;

            assert_eq!(db.get_total_progress()?, 5);

            Ok(())
      }

      #[test]
      fn test_get_total_question_count() -> Result<()> {
            let db = DB::new_in_memory()?;
            let right_answer = "0";
            let false_answers = vec!["1", "2", "3"];
            db.insert(1, "nan", right_answer, false_answers.clone())?;
            db.insert(2, "nan", right_answer, false_answers.clone())?;

            assert_eq!(db.get_total_question_count()?, TOTAL_COUNT_TRYS_PER_QUESTION * 2);

            Ok(())
      }

      #[test]
      fn test_is_empty() -> Result<()> {
            let db = DB::new_in_memory()?;
            assert!(db.is_empty()?);
            db.insert(1, "nan", "0", vec!["1", "2", "3"])?;
            assert!(!db.is_empty()?);
            Ok(())
      }

      #[test]
      fn test_clear() -> Result<()> {
            let db = DB::new_in_memory()?;
            assert!(db.is_empty()?);
            db.insert(1, "nan", "0", vec!["1", "2", "3"])?;
            assert!(!db.is_empty()?);
            db.clear()?;
            assert!(db.is_empty()?);

            Ok(())
      }

      #[test]
      fn test_clear_progress() -> Result<()> {
            let db = DB::new_in_memory()?;
            assert!(db.no_open_questions()?);
            db.insert(1, "nan", "0", vec!["1", "2", "3"])?;
            db.update_count_correct_answers(1, 3)?;
            assert!(db.no_open_questions()?);
            db.clear_progress()?;
            assert!(!db.no_open_questions()?);
            Ok(())
      }

      #[test]
      fn test_no_open_questions() -> Result<()> {
            let db = DB::new_in_memory()?;
            assert!(db.no_open_questions()?);
            db.insert(1, "nan", "0", vec!["1", "2", "3"])?;
            db.update_count_correct_answers(1, 3)?;
            assert!(db.no_open_questions()?);
            db.update_count_correct_answers(1, 2)?;
            assert!(!db.no_open_questions()?);
            Ok(())
      }
}
