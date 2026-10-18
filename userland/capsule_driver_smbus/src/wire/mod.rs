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

//! The SMBus wire. Its frame codec is driver.hda0's, mounted, under the
//! NSMB magic in `header`.

#[path = "../../../capsule_driver_hda/src/protocol/decode.rs"]
mod decode;
#[path = "../../../capsule_driver_hda/src/protocol/encode.rs"]
mod encode;
mod header;
mod ops;

pub use decode::{decode_request, refused};
pub use encode::{encode_response_header, write_status};
pub use header::{Request, HDR_LEN};
pub use ops::*;
