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

//! The client's wire as it ships, minus the call (a kernel primitive).

#[path = "../../../nonos_blk_client/src/wire/decode.rs"]
mod decode;
#[path = "../../../nonos_blk_client/src/wire/encode.rs"]
mod encode;
#[path = "../../../nonos_blk_client/src/wire/header.rs"]
mod header;

pub use decode::decode_reply;
pub use encode::{encode_request, rw_header};
pub use header::{HDR_LEN, SECTOR_SIZE, STATUS_LEN, VERSION};
