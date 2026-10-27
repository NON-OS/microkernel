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

//! net.sockets, which only a development build's local mirror reaches
//! (raw.rs): the call, its opcodes, and the body of a connect by name. No
//! guest's stream goes through it: a mixnet socket there carries an address
//! and refuses every name, so a guest's Nym stream goes through net.socks5
//! by name instead (routed/nym.rs).

#[path = "call.rs"]
pub mod call;
#[path = "host_body.rs"]
pub mod host_body;
#[path = "ops.rs"]
pub mod ops;
