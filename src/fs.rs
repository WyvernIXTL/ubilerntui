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
            Result,
            WrapErr
      }
};

use directories::BaseDirs;
use std::path::PathBuf;
use std::fs::create_dir_all;

use std::env;


pub fn get_local_dir<S: ToString>(dir_name: S) -> Result<PathBuf> {
      let path;
      if let Some(base_dir) = BaseDirs::new() {
            path = base_dir.data_local_dir().to_path_buf()
                  .join(env!("CARGO_PKG_NAME")).join(dir_name.to_string());

            create_dir_all(path.clone())
                  .wrap_err(format!("Failed creating folder: {:?}", path))
                  .suggestion("Check read and write rights of application for that folder.")?;
      } else {
            path = PathBuf::from("./").join(dir_name.to_string());
            create_dir_all(path.clone())?;
      }
      
      Ok(path)
}