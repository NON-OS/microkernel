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

//! Discovery's match and how many records it reads, as they ship. The
//! device list itself is the kernel's.

#[path = "../../capsule_driver_virtio_gpu/src/discover/constants.rs"]
mod constants;
#[path = "../../capsule_driver_virtio_gpu/src/discover/match_device.rs"]
pub mod match_device;

// A device past the 64th record was never seen; every wired driver and
// virtio-net read 128. Held when the proofs build.
const _: () = assert!(constants::MAX_DEVICES >= 128, "discovery reads too few records");
