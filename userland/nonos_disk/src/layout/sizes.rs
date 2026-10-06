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

//! The sizes the layout is built from, and the smallest disk they fit.

use nonos_disk_map::{DATA_FLOOR, SECTOR_SIZE};

/// One MiB in sectors. The data volume and the ESP start and end on it.
pub const MIB_SECTORS: u64 = (1 << 20) / SECTOR_SIZE as u64;

/// The EFI system partition: one GiB, so the image can grow across
/// releases without a repartition. The shipped image, a 14 MB loader and a
/// 90 MB kernel (`tests/every_write_is_whole_sectors.rs`), is a tenth of it.
pub const ESP_SECTORS: u64 = (1 << 30) / SECTOR_SIZE as u64;

/// The smallest data volume: one GiB, which holds the smallest pinned Qwen
/// model, 491,400,032 bytes
/// (`userland/capsule_linux/src/linux/file/models/pinned_qwen25.rs`), twice.
pub const DATA_MIN_SECTORS: u64 = (1 << 30) / SECTOR_SIZE as u64;

/// The smallest disk NONOS installs to: the 128 MiB below the data floor
/// (tables, store, plan, key header), the smallest data volume, the ESP,
/// and one MiB at the end for the backup table's 33 sectors and the ESP's
/// alignment. 2177 MiB, which a drive label calls 2.3 GB.
pub const MIN_DISK_SECTORS: u64 = DATA_FLOOR + DATA_MIN_SECTORS + ESP_SECTORS + MIB_SECTORS;
