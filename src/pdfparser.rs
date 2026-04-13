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
use once_cell::sync::Lazy;
use pdf_extract::extract_text_from_mem;
use regex::Regex;

/// Uses [pdf_extract] crate to [extract](pdf_extract::extract_text_from_mem) PDF read on location of `path`.
pub fn read_pdf_to_string(path: PathBuf) -> Result<String> {
    let bytes = read(path)?;
    print!("{}", extract_text_from_mem(&bytes)?);
    Ok(extract_text_from_mem(&bytes)?)
}

/// Trims, replaces bad line breaks and multiple spaces withing string.
macro_rules! to_trimmed_string {
    ($e:expr) => {{
        let s = ($e).trim();
        let s = s.replace("-\n", "");
        let s = s.replace("\n", " ");
        let s = s.replace("   ", " ");
        let s = s.replace("  ", " ");
        String::from(s)
    }};
}

fn extract_questions(reg: &Regex, s: &str) -> Vec<(usize, String, String, Vec<String>)> {
    reg.captures_iter(s)
        .map(|caps| {
            let id = usize::from_str_radix(&caps["id"], 10).unwrap();
            (
                id,
                to_trimmed_string!(&caps["question"]),
                to_trimmed_string!(&caps["a"]),
                vec![
                    to_trimmed_string!(&caps["b"]),
                    to_trimmed_string!(&caps["c"]),
                    to_trimmed_string!(&caps["d"]),
                ],
            )
        })
        .collect()
}

/// Uses regex to parse out all questions from string.
/// Supports two formats:
/// - UBI/Binnenschifffahrt: answers labeled a), b), c), d) with inline [id] bracket
/// - SRC/UKW-See: answers labeled 1), 2), 3), 4)
pub fn parse_pdf(s: String) -> Result<Vec<(usize, String, String, Vec<String>)>> {
    // UBI format: a), b), c), d) with [id] bracket inline in question
    static REG_UBI: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"(?ms)^\s?(?P<id>[0-9]{1,3})\.\s+(?P<question>.*?)\s+\[(?P<id2>[0-9]{1,3})\].*?a\)(?P<a>.*?)b\)(?P<b>.*?)c\)(?P<c>.*?)d\)(?P<d>.*?)\n$"
      ).unwrap()
    });

    // SRC format: 1), 2), 3), 4) — [id] bracket position is inconsistent
    static REG_SRC: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"(?ms)^\s?(?P<id>[0-9]{1,3})\.\s+(?P<question>.*?)1\)(?P<a>.*?)2\)(?P<b>.*?)3\)(?P<c>.*?)4\)(?P<d>.*?)\n$"
      ).unwrap()
    });

    let ubi_results = extract_questions(&REG_UBI, &s);
    if !ubi_results.is_empty() {
        return Ok(ubi_results);
    }

    Ok(extract_questions(&REG_SRC, &s))
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
            (
                128usize,
                "q part1 q part2".to_owned(),
                "correct answer".to_owned(),
                vec![
                    "wrong answer 1".to_owned(),
                    "wrong answer 2".to_owned(),
                    "wrong answer 3".to_owned(),
                ],
            ),
            (
                129,
                "q part1 q part2".to_owned(),
                "correct answer".to_owned(),
                vec![
                    "wrong answer 1".to_owned(),
                    "wrong answer 2".to_owned(),
                    "wrong answer 3".to_owned(),
                ],
            ),
            (
                127,
                "q part1 q part2".to_owned(),
                "correct answer part 1 correct answer part 2".to_owned(),
                vec![
                    "wrong answer 1 part 1 wrong answer 1 part 2 wrong answer 1 part 3".to_owned(),
                    "wrong answer 2 part 1 wrong answer 2 part 2".to_owned(),
                    "wrong answer 3 part 1 wrong answer 3 part 2".to_owned(),
                ],
            ),
        ];

        let res = parse_pdf(test_raw_string.to_owned())?;

        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn src_test_176() -> Result<()> {
        let raw_string = "


176.

q part1
q part2
q part3


 [176]

1)  correct answer

2)  wrong answer 1

3)  wrong answer 2

4)  wrong answer 3


 Gesamtfragenkatalog

        ";

        let expected = vec![(
            176usize,
            "q part1 q part2 q part3".to_owned(),
            "correct answer".to_owned(),
            vec![
                "wrong answer 1".to_owned(),
                "wrong answer 2".to_owned(),
                "wrong answer 3".to_owned(),
            ],
        )];

        let res = parse_pdf(raw_string.to_owned())?;

        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn src_test_177() -> Result<()> {
        let raw_string = "


177.

q part1
q part2
q part3


 [177]

1)

correct answer part1
correct answer part2



2)

wrong answer 1 part 1
wrong answer 1 part 2



3)

wrong answer 2 part 1
wrong answer 2 part 2



4)

wrong answer 3 part 1
wrong answer 3 part 2



 178.
        ";

        let expected = vec![(
            177usize,
            "q part1 q part2 q part3".to_owned(),
            "correct answer part1 correct answer part2".to_owned(),
            vec![
                "wrong answer 1 part 1 wrong answer 1 part 2".to_owned(),
                "wrong answer 2 part 1 wrong answer 2 part 2".to_owned(),
                "wrong answer 3 part 1 wrong answer 3 part 2".to_owned(),
            ],
        )];

        let res = parse_pdf(raw_string.to_owned())?;

        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn src_test_177_b() -> Result<()> {
        let raw_string = "


177.

q part1
q part2
q part3


 [177]

1)

correct answer part1
correct answer part2



2)

wrong answer 1 part 1
wrong answer 1 part 2



3)

wrong answer 2 part 1
wrong answer 2 part 2



4)

wrong answer 3 part 1
wrong answer 3 part 2


Gesamtfragenkatalog
        ";

        let expected = vec![(
            177usize,
            "q part1 q part2 q part3".to_owned(),
            "correct answer part1 correct answer part2".to_owned(),
            vec![
                "wrong answer 1 part 1 wrong answer 1 part 2".to_owned(),
                "wrong answer 2 part 1 wrong answer 2 part 2".to_owned(),
                "wrong answer 3 part 1 wrong answer 3 part 2".to_owned(),
            ],
        )];

        let res = parse_pdf(raw_string.to_owned())?;

        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn src_test_177_c() -> Result<()> {
        let raw_string = "


177.

q part1
q part2
q part3


 [177]

1)

correct answer part1
correct answer part2



2)

wrong answer 1 part 1
wrong answer 1 part 2



3)

wrong answer 2 part 1
wrong answer 2 part 2



4)

wrong answer 3 part 1
wrong answer 3 part 2


IV. 
        ";

        let expected = vec![(
            177usize,
            "q part1 q part2 q part3".to_owned(),
            "correct answer part1 correct answer part2".to_owned(),
            vec![
                "wrong answer 1 part 1 wrong answer 1 part 2".to_owned(),
                "wrong answer 2 part 1 wrong answer 2 part 2".to_owned(),
                "wrong answer 3 part 1 wrong answer 3 part 2".to_owned(),
            ],
        )];

        let res = parse_pdf(raw_string.to_owned())?;

        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn src_test_124() -> Result<()> {
        let raw_string = "

124.    question  [124]

1)  correct answer

2)  wrong answer 1

3)  wrong answer 2 (A1 bis A4)

4)  wrong answer 3



VII. 

        ";

        let expected = vec![(
            124usize,
            "question".to_owned(),
            "correct answer".to_owned(),
            vec![
                "wrong answer 1".to_owned(),
                "wrong answer 2 (A1 bis A4)".to_owned(),
                "wrong answer 3".to_owned(),
            ],
        )];

        let res = parse_pdf(raw_string.to_owned())?;

        assert_eq!(res, expected);

        Ok(())
    }
}
