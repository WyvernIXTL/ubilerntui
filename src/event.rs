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


use tracing::{trace_span, trace, debug_span};

use color_eyre::{
      Section, 
      eyre::{
            Report,
            Result,
            WrapErr,
            bail
      }
};

use crossterm::event::{self, KeyEvent, MouseEvent, Event};

use std::{
      sync::mpsc::{self, Receiver, Sender},
      thread,
      time::{Duration, Instant},
};


pub enum EventType {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16)
}

/// On creation spawns thread capturing key strokes, mouse movements and resizes of window.
/// Those are then accessable via [channel](Receiver<EventType>) saved in this struct.
pub struct  InputEventHandler {
      pub sender: Sender<EventType>, 
      pub receiver: Receiver<EventType>,
      handle: thread::JoinHandle<()>
}

impl InputEventHandler {
      /// Returns struct with channels. Spawns thread capturing key strokes, mouse movements and resizes of window.
      pub fn new(fps: u64) -> Self {
            let frametime = 1_000_000_000 / fps;
            let (sender, receiver) = mpsc::channel::<EventType>();

            let handle = {
                  let sender = sender.clone();

                  thread::spawn(move || {
                        let _trace_guard = trace_span!("Key, mouse and resize event polling loop.", frametime).entered();
                        loop {
                              if event::poll(Duration::from_nanos(frametime)).expect("Unable to poll for crossterm event.") {
                                    match event::read().expect("Unable to read crossterm event.") {
                                          Event::Resize(w, h) => {
                                                sender.send(EventType::Resize(w, h))
                                          },
                                          Event::Key(key_event) => {
                                                if key_event.kind == event::KeyEventKind::Press {
                                                      sender.send(EventType::Key(key_event))
                                                } else {
                                                    Ok(())
                                                }
                                          },
                                          Event::Mouse(mouse_event) => {
                                                sender.send(EventType::Mouse(mouse_event))
                                          },
                                          unknown_event => {
                                                trace!(?unknown_event, "Unknown event matched in even handler."); 
                                                Ok(()) 
                                          },
                                    }.expect("Failed sending in event channel.");
                              }
                        }
                  })
            };

            Self { sender: sender, receiver: receiver, handle: handle }
      }

      /// Get next key stroke/mouse event/resize event from channel.
      pub fn get(&mut self) -> Result<EventType> {
            Ok(self.receiver.recv()?)
      }
}