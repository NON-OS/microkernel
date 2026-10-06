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

//! Host proofs for net.dhcp.client's request header decode. Any process holding the
//! endpoint can send it any bytes; the `#[path]` includes pull in the real
//! decode and the reply it answers a refused frame with, under the `crate::`
//! paths their files name each other by.

// The wire's errnos, magic, ops and limits, which the decode names.
#[path = "../../capsule_net_dhcp/src/protocol/mod.rs"]
pub mod protocol;

// The header decode and the reply; not the handlers or the receive loop.
pub mod server;

#[cfg(test)]
mod refusal_tests;
