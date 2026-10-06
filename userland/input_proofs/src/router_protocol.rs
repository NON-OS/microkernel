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

//! The input router's request wire, file for file as
//! capsule_input_router/src/protocol/mod.rs lists it, less `delivery`: that
//! one encodes events out to subscribers and names the kernel's InputEvent,
//! which nothing here links.

#[path = "../../capsule_input_router/src/protocol/decode.rs"]
mod decode;
#[path = "../../capsule_input_router/src/protocol/encode.rs"]
mod encode;
#[path = "../../capsule_input_router/src/protocol/errno.rs"]
mod errno;
#[path = "../../capsule_input_router/src/protocol/header.rs"]
mod header;
#[path = "../../capsule_input_router/src/protocol/limits.rs"]
mod limits;
#[path = "../../capsule_input_router/src/protocol/ops.rs"]
mod ops;
#[path = "../../capsule_input_router/src/protocol/read_u32.rs"]
mod read_u32;

pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{E_ACCES, E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION, E_INVAL};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{IPC_PAYLOAD_MAX, STATUS_LEN, SUBSCRIBE_REQ_LEN};
pub use ops::{OP_GRAB_RELEASE, OP_GRAB_REQUEST, OP_HEALTHCHECK, OP_SUBSCRIBE};
pub use read_u32::read_u32;
