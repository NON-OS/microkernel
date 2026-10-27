// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The window's one channel: text frames to and from the simplex-chat core's
//! WebSocket chat API. The window never holds a network socket; what carries
//! the frames is a `Transport`.
//!
//! Two exist. `Transcript` replays a recorded session so the window runs and
//! shows real-shaped data with no core. `CoreChannel` is the place the NØNOS
//! channel to simplex.core goes (the design notes-
//! simplex-split.md); it is a stub until the box rule that serves the core's
//! port to this capsule alone exists.

pub mod core_channel;
pub mod echo;
pub mod open;
pub mod replay;
pub mod script;
pub mod transcript;

use alloc::string::String;

pub trait Transport {
    /// Sends one command frame, `{"corrId":"<id>","cmd":"<command>"}`.
    fn send(&mut self, frame: &str);
    /// The next frame from the core, if one is waiting.
    fn recv(&mut self) -> Option<String>;
    /// Called once per window tick, so a transport can keep time.
    fn tick(&mut self) {}
    /// Whether frames can flow now.
    fn up(&self) -> bool;
    /// A word for the window's header saying what carries the frames.
    fn label(&self) -> &'static str;
}
