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

//! The IPC framing of net.nym that stands alone: the records a batched read
//! answers with, the sizes a session's buffers are counted in, and the
//! request header's magic, errnos, ops and limits.

#[path = "../../capsule_net_nym/src/protocol/batch.rs"]
pub mod batch;

#[allow(dead_code)]
#[path = "../../capsule_net_nym/src/protocol/errno.rs"]
mod errno;
#[path = "../../capsule_net_nym/src/protocol/header.rs"]
mod header;
#[allow(dead_code)]
#[path = "../../capsule_net_nym/src/protocol/limits.rs"]
pub mod limits;
#[allow(dead_code)]
#[path = "../../capsule_net_nym/src/protocol/ops.rs"]
mod ops;

pub use errno::{E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION};
pub use header::MAGIC;
pub use limits::{IPC_PAYLOAD_MAX, WIRE_PACKET_MAX};
pub use ops::OP_RECV_BATCH;
