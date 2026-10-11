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

//! The block wire (PLAN.md, section 4), as this capsule serves it.

mod decode;
mod encode;
mod endpoint;
mod errno;
mod geometry;
mod header;
mod identify;
mod limits;
mod ops;
mod unit;

pub use decode::decode_request;
pub use encode::{encode_response_header, write_status};
pub use endpoint::KERNEL_REPLY_ENDPOINT;
pub use errno::{E_ACCES, E_INVAL, E_MSGSIZE, E_NODEV, E_NXIO, E_ROFS};
pub use geometry::{encode_geometry, MEDIA_PRESENT, READ_ONLY, REMOVABLE};
pub use header::{Request, HDR_LEN, RESP_HDR_LEN};
pub use identify::{encode_identify, MEDIUM_EMMC, MEDIUM_SD};
pub use limits::{GEOMETRY_LEN, RW_HEADER_LEN, STATUS_LEN};
pub use ops::{OP_FLUSH, OP_GEOMETRY, OP_HEALTHCHECK, OP_IDENTIFY, OP_READ, OP_WRITE};
pub use unit::serves_unit;
