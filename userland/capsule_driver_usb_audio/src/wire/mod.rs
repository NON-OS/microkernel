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

//! The NHDA wire driver.hda0 serves, so the audio server plays through
//! this driver with the client it already has. The header, the frame codec,
//! the errors and the output status are mounted from that driver, so the two
//! cannot drift; the ops and lengths here are the subset a USB device takes.

#[path = "../../../capsule_driver_hda/src/protocol/decode.rs"]
mod decode;
#[path = "../../../capsule_driver_hda/src/protocol/encode.rs"]
mod encode;
#[path = "../../../capsule_driver_hda/src/protocol/errno.rs"]
mod errno;
#[path = "../../../capsule_driver_hda/src/protocol/header.rs"]
mod header;
mod limits;
mod ops;
#[path = "../../../capsule_driver_hda/src/protocol/output.rs"]
mod output;

pub use decode::{decode_request, refused};
pub use encode::{encode_response_header, write_status};
pub use errno::{E_AGAIN, E_INVAL, E_NODEV, E_OK};
pub use header::{Request, HDR_LEN, RESP_HDR_LEN};
pub use limits::{MAX_PCM_CHUNK, OUTPUT_STATUS_PAYLOAD_LEN, STATUS_LEN};
pub use ops::*;
pub use output::{write_output_status, OutputStatus, OUT_HEADPHONE, OUT_LINE, OUT_SPEAKER};
