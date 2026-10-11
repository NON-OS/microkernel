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

//! The gfx wire (NVGP) as driver.virtio_gpu0 serves it: its header, frame
//! codec and field reads are mounted from that driver, so the two cannot
//! drift; the ops and lengths below are the subset a scanout without a GPU
//! answers.

#[path = "../../../capsule_driver_virtio_gpu/src/protocol/decode.rs"]
mod decode;
#[path = "../../../capsule_driver_virtio_gpu/src/protocol/encode.rs"]
mod encode;
#[path = "../../../capsule_driver_virtio_gpu/src/protocol/header.rs"]
mod header;
mod ops;
#[path = "../../../capsule_driver_virtio_gpu/src/protocol/read.rs"]
mod read;

pub use decode::{parse, refused};
pub use encode::{response_header, write_status};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use ops::*;
pub use read::{le_u32, le_u64};
