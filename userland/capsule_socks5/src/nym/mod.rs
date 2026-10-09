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

//! Carrying SOCKS streams over the mixnet.

mod address;
mod base58;
mod batch;
mod bind;
mod bootstrap;
mod choose;
mod discover;
mod exit;
mod recv;
mod send;
mod session;
mod watch;

pub use address::parse_address;
pub use bootstrap::{bootstrap_exit, BOOTSTRAP_EXITS};
pub use exit::{
    exit, note_answered, note_delivered, rotate_if_silent, rotations, set_exit, trying_another, Exit,
};
pub use recv::{recv_batch, Delivery, E_NO_SESSION};
pub use send::{connect_request, send_through_mixnet, SendError};
pub use session::{open_session, reset_session, session};
