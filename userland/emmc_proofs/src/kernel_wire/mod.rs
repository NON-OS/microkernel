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

//! The kernel's block wire client (src/hardware/block_device/wire), the
//! parts the capsule must agree with: the header constants, the GEOMETRY
//! decoder and the status names. Only the tests read it.

#![cfg(test)]

#[path = "../../../../src/hardware/block_device/wire/decode.rs"]
pub mod decode;
#[path = "../../../../src/hardware/block_device/wire/error.rs"]
pub mod error;
#[path = "../../../../src/hardware/block_device/wire/frame.rs"]
pub mod frame;
#[path = "../../../../src/hardware/block_device/wire/header.rs"]
pub mod header;
#[path = "../../../../src/hardware/block_device/wire/identity.rs"]
pub mod identity;
#[path = "../../../../src/hardware/block_device/wire/status.rs"]
pub mod status;

#[cfg(test)]
mod identify_tests;
#[cfg(test)]
mod status_tests;
#[cfg(test)]
mod tests;
