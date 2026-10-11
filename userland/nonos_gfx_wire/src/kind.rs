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

//! The adapter kinds CAPS names. Logged, never branched on: the flags say
//! what a driver can do. 1, 2, 3 and 5 are section 12's; 6 on are G's.

pub const VIRTIO_GPU: u16 = 1;
pub const DISPI: u16 = 2;
pub const SVGA_II: u16 = 3;
pub const SYNTHVID: u16 = 5;
/// The boot framebuffer firmware left (GOP, or a device tree's
/// simple-framebuffer), served as one fixed mode.
pub const BOOT_FB: u16 = 6;
/// Intel's display engine, driven natively.
pub const INTEL: u16 = 7;
/// AMD's display engine, driven natively.
pub const AMD: u16 = 8;
