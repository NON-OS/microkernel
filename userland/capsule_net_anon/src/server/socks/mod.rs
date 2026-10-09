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

//! SOCKS5 over IPC into the Anyone network.
//!
//! The browser speaks RFC 1928 as bytes to this service, as it does to
//! net.socks5 for Nym, and the frames share the service port with the API:
//! an API request opens with the magic, a SOCKS frame with 0 to 4.
//! Names travel to the exit to resolve, so no lookup leaves this machine.

mod answer;
mod anyone;
mod anyone_open;
mod anyone_stream;
mod conv;
mod frame;
mod front;
mod kept;
mod progress;
mod relay;
mod rep;
mod reply;
mod request;
mod stage;
mod stages;
mod tunnel;
mod turn;
mod who;
mod wire;

pub use answer::{answer, reap};
pub use front::Front;
