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

//! The broadcast deadline: while a core's own timer is stopped in an idle
//! state, one memory-mapped timer frame outside every core holds the
//! earliest deadline of the cores asleep, and its interrupt wakes each one
//! that is due. With no such frame, no state that stops the timer is used.

pub mod book;
mod count;
mod frame;
pub mod halves;
mod hand;
mod handler;
mod start;
mod state;

pub use hand::{hand_off, owner, ready, take_back};
pub use start::start;
pub use state::describe;
