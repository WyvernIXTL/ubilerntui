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


use std::time::{Instant, Duration};
use std::thread;

pub struct FpsTimer {
      first: Instant,
      frametime: Duration
}

impl FpsTimer {
      pub fn new(fps: u64) -> Self {
            Self { first: Instant::now(), frametime: Duration::from_nanos(1_000_000_000/fps) }
      }

      pub fn timeout(&mut self) {
            let second = Instant::now();
            let duration = second.duration_since(self.first);
            if duration < self.frametime {
                  let difference = self.frametime - duration;
                  thread::sleep(difference);
            }
            self.first = Instant::now();
      }
}