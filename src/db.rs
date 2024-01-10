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

use directories::BaseDirs;
use std::path::PathBuf;
use std::fs::{
      self,
      read_dir, 
      create_dir_all,
};

use std::env;


use rusqlite::Connection;

use crate::app::QuestionAnswer;


pub struct DB {
      pub db: Connection
}

impl DB {
      pub fn new(db_dir_name: &str , application_dir_name: &str) -> Result<Self> {
            let db_path;
            if let Some(base_dir) = BaseDirs::new() {
                  let appdata_dir_buf = base_dir.data_local_dir().to_path_buf();
                  let db_dir_path_buf = appdata_dir_buf.join(application_dir_name).join(db_dir_name);
                  create_dir_all(db_dir_path_buf.clone())
                        .wrap_err(format!("Failed creating folder for db: {:?}", db_dir_path_buf))
                        .suggestion("Check read and write rights of application for that folder.")?;

                  db_path = db_dir_path_buf.join("ublerndb.db3");

            } else {
                  create_dir_all("./db")?;
                  db_path = PathBuf::from("./db/ublerndb.db3");
            }

            let db = Connection::open(db_path)?;

            db.execute(
                  "CREATE TABLE IF NOT EXISTS questions (
                        id                            INTEGER PRIMARY KEY,
                        question                      TEXT,     
                        answers_0                     TEXT,
                        answers_1                     TEXT,
                        answers_2                     TEXT,
                        answers_3                     TEXT,
                        correctly_answered            INTEGER
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
            let (id, question, answers_0, answers_1, answers_2, answers_3, correctly_answered):
                  (usize, String, String, String, String, String, usize) = self.db.query_row(
                  "SELECT id, question, answers_0, answers_1, answers_2, answers_3, correctly_answered
                  FROM questions
                  WHERE correctly_answered < 3
                  ORDER BY RANDOM()
                  LIMIT 1"
                  , (), |f| {
                        Ok(
                              (f.get(0)?, f.get(1)?, f.get(2)?, f.get(3)?, f.get(4)?, f.get(5)?, f.get(6)?)
                        )
                  }
            )?;

            let possible_answers = vec![answers_0, answers_1, answers_2, answers_3];

            Ok(
                  QuestionAnswer {
                        id,
                        question,
                        possible_answers,
                        right_answer: 0,
                        user_answer: None,
                        count_correctly_answered: correctly_answered
                  }
            )
      }
}


#[cfg(test)]
mod tests {
      use super::*;

      #[test]
      fn test_insertion_and_read_single() -> Result<()> {
            let db = DB::new("db_test", env!("CARGO_PKG_NAME"))?;
            let right_answer = "0";
            let false_answers = vec!["1", "2", "3"];
            
            db.insert(1, "nan", right_answer, false_answers.clone())?;

            let q = db.get_random()?;

            assert_eq!(q.id, 1);
            assert_eq!(q.right_answer, 0);
            assert_eq!(q.possible_answers[0], "0");
            assert_eq!(q.possible_answers[1..4], false_answers);
            assert_eq!(q.question, "nan");

            db.db.execute("DROP TABLE questions", ())?;

            Ok(())
      }
}
