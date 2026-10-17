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
//! The driver's bring-up steps that read the host's displays: the scanouts
//! and output 0's EDID. The claim, register grants and queue mapping talk
//! to the kernel's broker and stay out.

#[path = "../../../capsule_driver_virtio_gpu/src/setup/edid.rs"]
pub mod edid;
#[path = "../../../capsule_driver_virtio_gpu/src/setup/scanouts.rs"]
pub mod scanouts;
