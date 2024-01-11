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


use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fs::remove_file;


use color_eyre::eyre::Result;
use rusqlite::Connection;


use crate::app::QuestionAnswer;
use crate::fs::get_local_dir;


const DB_NAME: &str = "ublerndb.sqlite3";


pub struct DB {
      pub db: Connection
}

impl DB {
      pub fn new(db_dir_name: &str ) -> Result<Self> {
            let db_path = get_local_dir(db_dir_name)?.join(DB_NAME);

            let db = Connection::open(db_path)?;

            db.execute(
                  "CREATE TABLE IF NOT EXISTS questions (
                        id                            INTEGER PRIMARY KEY,
                        question                      TEXT NOT NULL,     
                        answers_0                     TEXT NOT NULL,
                        answers_1                     TEXT NOT NULL,
                        answers_2                     TEXT NOT NULL,
                        answers_3                     TEXT NOT NULL,
                        correctly_answered            INTEGER NOT NULL
                  )",
                  ()
            )?;

            Ok(Self { db: db })
      }

      pub fn insert<S: ToString>(&self, id: usize, question: S, right_answer: S, false_answers: Vec<S>) -> Result<()> {
            debug_assert!(false_answers.len() == 3);

            self.db.execute(
                  "INSERT INTO questions (id, question, answers_0, answers_1, answers_2, answers_3, correctly_answered)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                  (&id, &question.to_string(), &right_answer.to_string(), &false_answers[0].to_string(), &false_answers[1].to_string(), &false_answers[2].to_string(), 0)
            )?;

            Ok(())
      }

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
}


#[cfg(test)]
mod tests {
      use super::*;

      fn del_db_file() -> Result<()> {
            let path = get_local_dir("db_test")?.join(DB_NAME);
            if Path::new(&path).exists() {
                  remove_file(path)?;
            }
            Ok(())
      }

      #[test]
      fn test_insertion_and_read_single() -> Result<()> {
            del_db_file()?;
            let db = DB::new("db_test")?;
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
}
