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

macro_rules! to_trimmed_string {
      ($e:expr) => {
            {
                  let s = ($e).trim();
                  let s = s.replace("-\n", "");
                  let s = s.replace("\n", " ");
                  let s = s.replace("   ", " ");
                  let s = s.replace("  ", " ");
                  String::from(s)
            }
      };
}

pub fn parse_pdf(s: String) -> Result<Vec<(usize, String, String, Vec<String>)>> {
      static REG: Lazy<Regex> = Lazy::new(|| Regex::new(
            r"(?ms)^\s?(?P<id>[0-9]{1,3})\.\s+(?P<question>.*?)\s+\[(?P<id2>[0-9]{1,3})\].*?a\)(?P<a>.*?)b\)(?P<b>.*?)c\)(?P<c>.*?)d\)(?P<d>.*?)\n$"
      ).unwrap());
      let res = REG.captures_iter(&s).map(|caps| {
            let id = usize::from_str_radix(&caps["id"], 10).unwrap();
            (
                  id, 
                  to_trimmed_string!(&caps["question"]),
                  to_trimmed_string!(&caps["a"]),
                  vec![
                        to_trimmed_string!(&caps["b"]),
                        to_trimmed_string!(&caps["c"]),
                        to_trimmed_string!(&caps["d"])
                  ]
            )
      }).collect();

      Ok(res)
}


#[cfg(test)]
mod tests {
      use super::*;
      use pretty_assertions::assert_eq;


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