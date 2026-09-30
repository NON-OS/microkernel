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
 * Which disk the machine booted from. The loader records the partition it
 * was read from, as the firmware named it; a disk whose table holds that
 * partition is the boot media. With no record, a disk whose ESP holds the
 * running loader byte for byte is. The installers leave that disk out.
 */

mod fat;
mod fat_chain;
mod fat_dir;
mod identify;
mod loader_file;
mod read;
mod record;
mod table;
mod volumes;

pub use identify::{is_boot_media, BootEvidence};
pub use record::{
    BootPartition, BOOT_MEDIA_LEN, SIGNATURE_GUID, SIGNATURE_MBR, TABLE_GPT, TABLE_MBR,
};
