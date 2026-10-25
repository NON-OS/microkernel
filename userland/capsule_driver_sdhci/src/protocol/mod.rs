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

//! The block wire (PLAN.md, section 4), as this capsule serves it: the
//! shared capsule side (`nonos_blk_wire`), with this capsule's endpoint and
//! units.

mod endpoint;
mod unit;

pub use endpoint::KERNEL_REPLY_ENDPOINT;
pub use nonos_blk_wire::{
    body_fits, decode_request, encode_geometry, encode_identify, encode_response_header,
    write_status, Request, E_ACCES, E_INVAL, E_MSGSIZE, E_NODEV, E_NXIO, E_ROFS, GEOMETRY_LEN,
    HDR_LEN, MEDIA_PRESENT, MEDIUM_EMMC, MEDIUM_SD, OP_FLUSH, OP_GEOMETRY, OP_HEALTHCHECK,
    OP_IDENTIFY, OP_READ, OP_WRITE, READ_ONLY, REMOVABLE, RESP_HDR_LEN, RW_HEADER_LEN, STATUS_LEN,
};
pub use unit::serves_unit;
