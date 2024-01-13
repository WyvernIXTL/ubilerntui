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


use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Instant, Duration};
use std::thread;
use std::env::{self, var};
use std::path::PathBuf;
use std::io;
use std::io::Write;

use tracing::{
      debug, 
      error, 
      info, 
      warn, 
      trace, 
      span, 
      Level, 
      instrument, 
      debug_span,
      error_span,
      trace_span
};
use color_eyre::{
      Section, 
      eyre::{
            Report,
            Result,
            WrapErr,
            bail
      }
};

use colored::*;

pub mod logging;
use logging::start_tracing;

pub mod tui;
use tui::Tui;

pub mod hooks;
use hooks::eyre_term_exit_hook;

pub mod app;
use app::App;

pub mod event;

pub mod update;

pub mod ui;

pub mod fpslimiter;

pub mod db;
use db::DB;

pub mod fs;

pub mod pdfparser;
use pdfparser::{parse_pdf, read_pdf_to_string};

pub mod argparsing;
use argparsing::commands_and_flags;


const LOG_DIR_NAME: &str = "logs";
const DB_DIR_NAME: &str = "db";
const FPS: u64 = 120;


fn main() -> Result<()> {
      let entered_alternative_mode = Arc::new(AtomicBool::new(false));
      eyre_term_exit_hook(entered_alternative_mode.clone())?;

      start_tracing(LOG_DIR_NAME)?;

      info!(
            name = %env!("CARGO_PKG_NAME"),
            version = %env!("CARGO_PKG_VERSION"),
            repo = %env!("CARGO_PKG_REPOSITORY"),
            authors = %env!("CARGO_PKG_AUTHORS"),
            os = %env::consts::OS,
            "program_and_env_info"
      );

      let db = DB::new(DB_DIR_NAME)?;

      let mut commands = commands_and_flags();
      let matches = commands.clone().get_matches();

      match matches.subcommand() {
            Some(("lade", sub_matches)) => {
                  if !db.is_empty()? {
                        if !yn_inquire("Das Laden wird die alten Daten überschreiben. Trotzdem tun?")? {
                              return Ok(());
                        }
                        db.clear()?;
                  }
                  let path_str = sub_matches.get_one::<String>("PFAD").expect("required");
                  let path = PathBuf::from(path_str);
                  let mut count = 0;
                  for q in parse_pdf(read_pdf_to_string(path)?)? {
                        db.insert_tuple(q)?;
                        count += 1;
                  }
                  let res_msg = format!("{count} Fragen erfolgreich aus der PDF-Datei geladen.").green();
                  println!("{}", res_msg);
            },
            Some(("loesche", sub_matches)) => {
                  match (*sub_matches).subcommand() {
                        Some(("fragen", _)) => {
                              if !yn_inquire("Wollen Sie die Fragen wirklich aus der Datenbank löschen?")? {
                                    return Ok(());
                              }
                              db.clear()?;
                              info!("Deleted data in question table.");
                              println!("{}", "Fragen erfolgreich aus der Datenbank entfernt.".green());
                        },
                        Some(("fortschritt", _)) => {
                              if !yn_inquire("Wollen Sie den Fortschritt wirklich aus der Datenbank löschen?")? {
                                    return Ok(());
                              }
                              db.clear_progress()?;
                              info!("Deleted progress in question table.");
                              println!("{}", "Lern-Fortschritt erfolgreich aus der Datenbank entfernt.".green());
                        },
                        _ => unimplemented!()
                  }
            },
            _ => {
                  if db.is_empty()? {
                        println!("Bitte laden Sie das dazugehörige PDF. Mehr dazu in der Anleitung:");
                        commands.print_long_help()?;
                  } else if db.no_open_questions()? {
                        println!("Sie haben bereits alle Fragen gelernt! 
                        Sie können diese nochmal lernen via ubilerntui loesche fortschritt.");
                        commands.print_help()?;
                  } else {
                        start_learn_tui(entered_alternative_mode, &db)?;
                  }
            }
      }
      
      Ok(())
}

fn yn_inquire(what: &str) -> Result<bool> {
      loop {
            print!("{} {} ", what.yellow(), "Y/n:".yellow());
            io::stdout().flush()?;
            let mut s = "".to_owned();
            io::stdin().read_line(&mut s)?;
            let s = s.to_lowercase();
            let s = s.trim();
            if s == "y" || s == "j" {
                  return Ok(true);
            } else if s == "n" {
                  return Ok(false);
            }
      }
}

fn start_learn_tui(entered_alternative_mode: Arc<AtomicBool>, db: &DB) -> Result<()> {
      let first_question = db.get_random()?;
      let mut app = App::new(
            first_question, 
            db.get_total_progress()?, 
            db.get_total_question_count()?
      );
      app.question_answer.scramble();


      entered_alternative_mode.swap(true, Ordering::Relaxed);
      let mut term = Tui::new_with_term()?;
      term.enter()?;
      trace!("Entered alternative screen mode.");


      let event_handler = event::InputEventHandler::new(FPS);

      let main_span = trace_span!("Main Loop").entered();

      let mut fps_timer = fpslimiter::FpsTimer::new(FPS);
      loop {
            while let Ok(event) = event_handler.receiver.try_recv() {
                  update::update(event, &mut app, &db)?;
            }
            if app.exit {
                  break;
            }
            term.draw(&mut app)?;

            fps_timer.timeout();
      }
      main_span.exit();


      term.exit()?;

      Ok(())
}


