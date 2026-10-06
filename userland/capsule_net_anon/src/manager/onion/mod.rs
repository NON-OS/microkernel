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


//! Reaching an onion service: one lookup per stream being opened.
//!
//! A stream to a `.anyone` address is answered at once with a stream id, as
//! a stream to a host name is, and sits in Opening while its lookup runs
//! over the idle ticks: fetch the descriptor from a responsible HSDir over
//! BEGIN_DIR, set up a rendezvous point, introduce through one of the
//! service's introduction points, and on RENDEZVOUS2 add the service as a
//! fourth hop and send BEGIN. Nothing here waits inside a tick. A lookup
//! that fails ends its stream with a reason, as an exit's END would.

mod auth;
mod circuits;
mod directory;
mod inbound;
mod job;
mod open;
mod pow;
mod rendezvous;
mod tick;

pub use auth::{forget_client_key, set_client_key, KeyError};
pub(super) use inbound::inbound;
pub use job::OnionJob;
pub(super) use open::open;
pub use tick::tick;
