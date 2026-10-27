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

//! STUB: the NØNOS channel to simplex.core.
//!
//! The core runs in its own box with `-p PORT` and serves the chat API on
//! 127.0.0.1. The letter of 2026-10-27 asks S1 for a box grant,
//! `SERVE_ONE simplex.ui`, that serves that listen as one NØNOS endpoint only
//! this capsule may connect to. When it exists this type opens it, does the
//! WebSocket handshake, and moves text frames; until then it is never up and
//! drops what it is given, and the window runs on `Transcript`.

use alloc::string::String;

use super::Transport;

pub struct CoreChannel;

impl Transport for CoreChannel {
    fn send(&mut self, _frame: &str) {}

    fn recv(&mut self) -> Option<String> {
        None
    }

    fn up(&self) -> bool {
        false
    }

    fn label(&self) -> &'static str {
        "simplex.core"
    }
}
