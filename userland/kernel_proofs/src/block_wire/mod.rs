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

/*
 * The kernel's block-wire client, the parts that hold without a capsule:
 * reply statuses, the GEOMETRY payload, and which blocks a sector request
 * touches. Each file is included by path from src/hardware/block_device/wire.
 */

#[path = "../../../../src/hardware/block_device/wire/decode.rs"]
mod decode;
#[path = "../../../../src/hardware/block_device/wire/error.rs"]
#[allow(dead_code)]
mod error;
#[path = "../../../../src/hardware/block_device/wire/frame.rs"]
mod frame;
#[path = "../../../../src/hardware/block_device/wire/header.rs"]
#[allow(dead_code)]
mod header;
#[path = "../../../../src/hardware/block_device/wire/identity.rs"]
mod identity;
#[path = "../../../../src/hardware/nvme_capsule/client/lba_map.rs"]
#[allow(dead_code)]
mod lba_map;
#[path = "../../../../src/hardware/block_device/wire/span.rs"]
mod span;
#[path = "../../../../src/hardware/block_device/wire/status.rs"]
mod status;

mod frame_tests;
mod geometry_tests;
mod identity_tests;
mod span_tests;
mod status_tests;
mod table_tests;
