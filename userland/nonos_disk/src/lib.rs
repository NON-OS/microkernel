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

//! A whole NONOS disk, written sector by sector.
//!
//! ```text
//!   0 .. 34             protective MBR, primary GPT
//!   256 .. 65536        NONOS-STORE  the package store
//!   65536 .. 131072     NONOS-PLAN   the disk plan, then the key header
//!   131072 .. E         NONOS-DATA   the data volume the plan names
//!   E .. end - 33       NONOS-ESP    bootloader, kernel image, boot.cfg
//! ```
//!
//! The kernel reads the store, the plan and the key header at fixed sectors
//! (`nonos_disk_map`), so they stay there and the table names them as
//! partitions; the ESP goes last, where it meets nothing. The store holds
//! what [`gather`] carried over, the plan names the volume and no imports,
//! the key header is cleared so the first boot keys the volume with the
//! TPM, and the volume's header ring is zeroed so that boot formats it.
//!
//! A [`BlockSink`] takes `(lba, bytes)`: a block driver in the installers,
//! memory in the host tests, so the disk the tests read is the one written.

#![no_std]

extern crate alloc;

mod boot_media;
mod carry;
mod crc32;
mod describe;
mod fat32;
mod gpt;
mod guid;
mod image;
mod layout;
mod plan_sector;
mod session;
mod sink;
mod store;
mod writer;

pub use boot_media::{
    is_boot_media, BootEvidence, BootPartition, BOOT_MEDIA_LEN, SIGNATURE_GUID, SIGNATURE_MBR,
    TABLE_GPT, TABLE_MBR,
};
pub use carry::{gather, Carried, CarrySource};
pub use describe::{describe, size_text, Row};
pub use fat32::{Geometry, Node, PlanError, WriteVolumeError};
pub use gpt::written_by_nonos;
pub use guid::Guid;
pub use image::{NonosImage, STARTUP_NSH};
pub use layout::{Extent, Layout, Region, DATA_MIN_SECTORS, ESP_SECTORS, MIN_DISK_SECTORS};
pub use plan_sector::plan_sector;
pub use session::{Ids, Plan, Progress, Session, Verifier, ENTROPY_BYTES};
pub use sink::{BlockSink, SinkError, SECTOR_SIZE};
pub use store::{StoreBuilder, StoreError, StoreImage};
pub use writer::{install, verify, FileRun, Receipt, WriteError};
