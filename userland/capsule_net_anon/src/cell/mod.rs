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

//! Cells, the unit everything on a link is made of.

mod commands;
mod fixed;
mod geometry;
mod read;
mod relay;
mod var;

pub use commands::{
    CELL_AUTH_CHALLENGE, CELL_CERTS, CELL_CREATE2, CELL_CREATED2, CELL_DESTROY, CELL_NETINFO,
    CELL_RELAY, CELL_RELAY_EARLY, CELL_VERSIONS, CELL_VPADDING, HANDSHAKE_NTOR, RELAY_BEGIN,
    RELAY_CONNECTED, RELAY_DATA, RELAY_END, RELAY_EXTEND2, RELAY_EXTENDED2, RELAY_SENDME,
    RELAY_TRUNCATED,
};
pub use fixed::Cell;
pub use geometry::{PAYLOAD_BYTES, RELAY_BODY_BYTES};
pub use read::{parse, parse_versions, Frame};
pub use relay::{body, pack, put_digest, take_digest, unpack, RelayHeader};
pub use var::VarCell;
