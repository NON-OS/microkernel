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

//! A bootable NONOS disk, written sector by sector.
//!
//! An installed NONOS is three files on an EFI system partition: the
//! bootloader at `EFI/BOOT/BOOTX64.EFI`, the attested kernel image at
//! `EFI/nonos/kernel.bin`, and its `boot.cfg`. Firmware finds the first by
//! its fixed path, the bootloader finds the rest on the volume it was loaded
//! from. Nothing else is on the disk and nothing is ever written back to it,
//! which is what lets the installed system stay the system that was verified.
//!
//! This crate lays the disk out and knows nothing about where the sectors go:
//! a [`BlockSink`] takes `(lba, bytes)`. The installer capsule backs it with a
//! block driver; the host tests back it with memory and mount the result with
//! mtools. One writer, so the disk the tests read back is the disk the
//! installer writes.

#![no_std]

extern crate alloc;

mod crc32;
mod fat32;
mod gpt;
mod guid;
mod image;
mod session;
mod sink;
mod writer;

pub use fat32::{Geometry, Node, PlanError, WriteVolumeError};
pub use gpt::{Layout, ESP_FIRST_LBA, PARTITION_NAME};
pub use guid::Guid;
pub use image::{NonosImage, STARTUP_NSH};
pub use session::{Plan, Progress, Session, Verifier};
pub use sink::{BlockSink, SinkError, SECTOR_SIZE};
pub use writer::{install, verify, FileRun, Receipt, WriteError};
