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

use directories::BaseDirs;
use std::path::PathBuf;
use std::fs::{
      self,
      read_dir, 
      create_dir_all, 
      OpenOptions,
      File
};
use chrono::Local;
use std::io;


const MAX_LOG_FILES: usize = 10;


pub fn start_tracing(log_dir_name: &str, application_dir_name: &str) -> Result<()> {
      let log_file = create_new_logfile(log_dir_name, application_dir_name, MAX_LOG_FILES)
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

fn create_new_logfile(log_dir_name: &str, application_dir_name: &str, max_num_log_files: usize) -> Result<File> {
      let logs_path: PathBuf;

      let date = Local::now();
      let date_string = format!("{}", date.format("%Y-%m-%d--%H-%M-%S"));

      if let Some(base_dir) = BaseDirs::new() {
            let appdata_dir_buf = base_dir.data_local_dir().to_path_buf();
            let log_dir_path_buf = appdata_dir_buf.join(application_dir_name).join(log_dir_name);
            create_dir_all(log_dir_path_buf.clone())
            .wrap_err(format!("Failed creating folder for logs: {:?}", log_dir_path_buf))
            .suggestion("Check read and write rights of application for that folder.")?;
            logs_path = log_dir_path_buf.join(date_string + ".json");

            prune_logs(log_dir_path_buf.clone(), max_num_log_files)
                  .wrap_err("Failed pruning logs.")?;
      } else {
            create_dir_all("./logs")?;
            logs_path = PathBuf::from("./logs/".to_owned() + &date_string + ".json");
      }

      let log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(logs_path.clone())
            .wrap_err(format!("Failed opening logfile at {:?}", logs_path))
            .suggestion("Check what applications have write and read access for that folder.")?;

      Ok(log_file)
}

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