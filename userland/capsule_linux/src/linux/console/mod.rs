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

//! The terminal a family may run on: what it typed, its settings and its
//! size. A run started with `cli` is on one from the start; any other is
//! on one once the kernel says a stream of the personality reaches a
//! terminal. What is typed and what is printed are the person's own, so
//! nothing here ever reaches a log.

mod input;
mod input_ready;
mod queue;
mod queue_piece;
mod queue_take;
mod say;
mod state;
mod termios;
mod tty;
mod wipe;

pub use input::read;
pub use input_ready::{bits, queued};
pub use say::say;
pub use state::{attached, enter, flush};
pub use termios::{set_termios, termios};
pub use tty::{private, size};
pub use wipe::wipe;
