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

//! The module paths the included cell and circuit source expects, pointed at
//! the real files rather than at copies of them.

#[path = "../../../capsule_net_anon/src/cell/geometry.rs"]
pub mod geometry;

#[path = "../../../capsule_net_anon/src/cell/commands.rs"]
pub mod commands;

#[path = "../../../capsule_net_anon/src/cell/fixed.rs"]
pub mod fixed;

#[path = "../../../capsule_net_anon/src/cell/relay/mod.rs"]
mod relay;

pub use commands::*;
pub use fixed::Cell;
pub use geometry::{PAYLOAD_BYTES, RELAY_BODY_BYTES, RELAY_HEADER_BYTES};
pub use relay::{body, pack, put_digest, take_digest, unpack, RelayHeader};
