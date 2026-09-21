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

//! The client side of `net.tcp`.
//!
//! Connections are opened by address, not by name. Relay addresses come out of
//! the consensus as four bytes, so there is nothing to resolve, and going
//! through `net.sockets` for a name lookup would add a dependency on the DNS
//! capsule that this transport does not need and should not have.
//!
//! This is the same client the mixnet capsule carries in its own
//! `tcp_client`, module for module. Neither is shared, because lifting it into
//! a crate means editing `capsule_net_nym`, which is a proven transport, and
//! that is a change to make on its own with its own boot. Recorded here so the
//! duplication is a known debt rather than a discovery.

mod envelope;
/*
 * Public so a caller can tell the refusals apart rather than printing a number.
 * The manager needs to distinguish "no route yet" from a service defect, and a
 * private module forced it to compare against a literal.
 */
pub mod errno;
mod ops;
mod send;
mod state;
mod wait;

pub use ops::{close, connect, recv};
pub use send::send_all;
pub use state::state;
pub use wait::wait_established;
