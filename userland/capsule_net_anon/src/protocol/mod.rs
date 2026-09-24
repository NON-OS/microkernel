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

//! The IPC surface `net.anon` serves.

mod errno;
mod limits;
mod ops;

pub use errno::{
    E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION, E_DIRECTORY_STALE, E_NO_CIRCUIT,
    E_NO_DIRECTORY, E_NO_LINK, E_NO_PATH, E_NO_STREAM, E_NO_TCP, E_OK, E_RX_EMPTY, E_STREAM_CLOSED,
    E_TABLE_FULL, E_WOULD_BLOCK,
};
pub use limits::{
    CIRCUIT_DIRTY_SECONDS, CIRCUIT_FAILURES_MAX, CIRCUIT_HIGH_WATER, CIRCUIT_MAX, HDR_LEN, HOPS,
    IPC_PAYLOAD_MAX, MAGIC, STREAM_HIGH_WATER, STREAM_MAX, VERSION,
};
pub use ops::{
    OP_BUILD_CIRCUIT, OP_CIRCUIT_PATH, OP_CLOSE_CIRCUIT, OP_CLOSE_STREAM, OP_HEALTHCHECK,
    OP_OPEN_STREAM, OP_RECV, OP_SEND, OP_STATUS, OP_SYNC_DIRECTORY,
};
