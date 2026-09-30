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

mod decode;
mod encode;
mod errno;
mod header;
mod limits;
mod ops;

pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{
    E_ACCES, E_AGAIN, E_BAD_OP, E_INVAL, E_IO, E_MSGSIZE, E_NODEV, E_NOTSUP, E_NO_MSC, E_NXIO,
    E_OVERFLOW, E_PHASE,
};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{
    BLK_HEADER_LEN, BLK_MAX_BYTES, BLK_MAX_SECTORS, BLOCK_BYTES, CBW_LEN, CSW_LEN,
    KERNEL_REPLY_ENDPOINT, MAX_BINDINGS, MAX_TRANSFER_BLOCKS, STATUS_LEN,
};
pub use ops::{
    OP_ACCEPT_CSW, OP_BLK_CAPACITY, OP_BLK_FLUSH, OP_BLK_READ, OP_BLK_WRITE, OP_BUILD_INQUIRY,
    OP_BUILD_READ10, OP_BUILD_READ_CAPACITY10, OP_BUILD_REQUEST_SENSE, OP_BUILD_TEST_UNIT_READY,
    OP_BUILD_WRITE10, OP_DECODE_CAPACITY, OP_DECODE_INQUIRY, OP_DECODE_SENSE, OP_GET_STATE,
    OP_HEALTHCHECK, OP_PROBE_CONFIG,
};
