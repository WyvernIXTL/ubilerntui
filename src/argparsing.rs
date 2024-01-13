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


use clap::{Command, arg};
use std::env;

use once_cell::sync::Lazy;


static LONG_HELP: Lazy<String> = Lazy::new(|| format!("{} v{} (c) {} {}

Eine TUI, um für die UBI-Prüfung zu lernen.

Anleitung:
1. Holen Sie sich den offiziellen Fragebogen des WSV.
   Beispielsweise über https://duckduckgo.com/?q=%2BUBI+Fragenkatalog+WSV+site%3Awsv.de+filetype%3Apdf&t=ffab&ia=web
2. Laden Sie das PDF in das Programm rein mit:
   ubilerntui lade ./UBI_Gesamtfragenkatalog.pdf
3. Trainieren Sie die Fragen mit:
   ubilerntui
4. Bestehen Sie die Prüfung :)
", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_LICENSE"))
);

/// Returns structure for `clap` to parse cli arguments.
/// 
/// This crate contains a single function returning `Command` struct from `clap`.
/// It is used to parse the command line arguments.
/// 
/// ```
/// let commands = commands_and_flags();
/// ```
pub fn commands_and_flags() -> Command {
      Command::new("ubilerntui")
      .about("Eine TUI, um für die UBI-Prüfung zu lernen.")
      .long_about(&*LONG_HELP)
      .args([
            arg!(--license "Prints license information."),
            arg!(--version "Prints version information.")
      ])
      .subcommands([
            Command::new("lade")
                  .about("Lädt eine UBI-Gesamtfragenkatalog-PDF-Datei in die interne Datenbank.")
                  .arg(arg!(<PFAD> "Pfad der PDF-Datei.")),
            Command::new("loesche")
                  .about("Löscht alle Fragen oder den Fortschritt aus der Datenbank.")
                  .subcommand_required(true)
                  .arg_required_else_help(true)
                  .subcommands([
                        Command::new("fragen").about("Löscht alle Fragen aus der Datenbank"),
                        Command::new("fortschritt").about("Löscht den Fortschritt aus der Datenbank.")
                  ])
      ])
}