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

//! Where the firmware's packets go while the station joins or is joined.
//! A received frame is parsed (`rx_data`) and kept, in order, for the join or
//! the link to read; a transmit response frees its queue's slots; a session
//! protection notification is remembered. The firmware may send any of these
//! while a host command waits for its reply, so they are routed the same way
//! then, and none is lost to the wait. The frame queue is bounded: a frame
//! arriving with it full is counted and dropped.
mod missed;
mod route;
mod state;

pub use state::{Inbox, Queues, INBOX_FRAMES};
