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

 
use color_eyre::{
      eyre::WrapErr, 
      Section, 
      eyre::Result
};

use tracing_subscriber::{
      fmt, 
      prelude::*, 
      filter::EnvFilter,
      fmt::format::FmtSpan
};
use tracing_error::ErrorLayer;

use std::path::PathBuf;
use std::fs::{
      self,
      read_dir,
      OpenOptions,
      File
};
use chrono::Local;
use std::io;

use crate::fs::get_local_dir;

const MAX_LOG_FILES: usize = 10;


/// If it does not exist, creates folder for logs, and starts structured json logging.
/// ```
/// start_tracing("logs")?;
/// ```
/// Should be called after eyre hooks have been installed.
/// One logfile per session is created.
pub fn start_tracing(log_dir_name: &str) -> Result<()> {
      let log_file = create_new_logfile(log_dir_name, MAX_LOG_FILES)
            .wrap_err("Failed creating a new log file.")
            .suggestion("Check permissions to write into appdata or similar folders or disable logging.")?;

      #[cfg(debug_assertions)]
      let span_log_level = FmtSpan::NONE;

      #[cfg(not(debug_assertions))]
      let span_log_level = FmtSpan::NONE;

      let subscriber = fmt::layer()
            .with_writer(log_file)
            .json()
            .with_span_events(span_log_level)
            .with_span_list(true)
            .with_file(true)
            .with_line_number(true)
            .with_filter(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO")));
      tracing_subscriber::registry().with(subscriber).with(ErrorLayer::default()).init();

      Ok(())
}

/// Creates and returns new logfile with unique name and returns it.
/// 
/// Also prunes old log files.
/// One lofile per call is created.
fn create_new_logfile(log_dir_name: &str, max_num_log_files: usize) -> Result<File> {
      let date = Local::now();
      let date_string = format!("{}", date.format("%Y-%m-%d--%H-%M-%S"));

      let log_dir_path = get_local_dir(log_dir_name)?;
      let log_file_path = log_dir_path.join(date_string + ".json");

      prune_logs(log_dir_path, max_num_log_files)
            .wrap_err("Failed pruning logs.")?;
            

      let log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_file_path.clone())
            .wrap_err(format!("Failed opening logfile at {:?}", log_file_path))
            .suggestion("Check what applications have write and read access for that folder.")?;

      Ok(log_file)
}

/// Removes old logfiles. 
/// 
/// `max_size` is the count of files the folder is to be pruned to.
/// The files are sorted by their name. Thus if the name has no connection with the age there'll be a huge problem.
fn prune_logs(dir: PathBuf, max_size: usize) -> Result<()> {

      let mut dir_entries = read_dir(dir.clone())
            .wrap_err(format!("Error while trying to iterate over dir {:?}", dir))
            .suggestion("Check what applications have read access for that folder.")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .wrap_err("Failed getting path of file while pruning logs.")
            .suggestion("This file might not exist anymore. Try again.")?;

      dir_entries.sort();
      dir_entries.reverse();

      let mut count = 0;
      for entry in dir_entries.iter() {
            count += 1;
            if count >= max_size {
                  fs::remove_file(entry)
                        .wrap_err("Failed deleting old log.")
                        .suggestion("Check access of application to appdata local or equivalent folder.")?;
            }
      }

      Ok(())
}