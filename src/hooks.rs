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


use std::panic;
use std::sync::{
      atomic::{AtomicBool, Ordering}, 
      Arc
};

use color_eyre::eyre::{
      self,
      Result
};

use tracing::error; 

use crate::tui::partial_exit;

pub fn eyre_term_exit_hook(exit_alternative_mode: Arc<AtomicBool>) -> Result<()> {
      let hook_builder = color_eyre::config::HookBuilder::default();
      let (panic_hook, eyre_hook) = hook_builder.into_hooks();

      let panic_hook = panic_hook.into_panic_hook();

      let exit_alternative_mode_info = exit_alternative_mode.clone();
      panic::set_hook(Box::new(move |panic_info| {
            error!(%panic_info);
            if exit_alternative_mode_info.load(Ordering::Relaxed) {
                  partial_exit().unwrap();
            }
            panic_hook(panic_info);
      }));

      // convert from a color_eyre EyreHook to a eyre ErrorHook
      let eyre_hook = eyre_hook.into_eyre_hook();
      let exit_alternative_mode_error = exit_alternative_mode.clone();
      eyre::set_hook(Box::new(move |error| {
            error!(%error);
            if exit_alternative_mode_error.load(Ordering::Relaxed) {
                  partial_exit().unwrap();
            }
            eyre_hook(error)
      }))?;

      Ok(())
}