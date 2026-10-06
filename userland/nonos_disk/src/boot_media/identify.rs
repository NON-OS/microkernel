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
 * The one question the installers ask of a disk. With a record, the disk
 * is the boot media when its table holds the recorded partition, and no
 * other evidence is weighed: a disk with a copy of the same loader on it,
 * a NONOS install of this very build, stays on the list. Without one, as
 * from a loader too old to leave it or a medium the firmware named no
 * partition on, the disk is the boot media when a volume on it holds the
 * running loader.
 */

use super::loader_file::holds_loader;
use super::record::BootPartition;
use super::table::table_names;
use super::volumes::volume_starts;
use crate::sink::BlockSink;

pub struct BootEvidence<'a> {
    pub partition: Option<BootPartition>,
    /* The loader file's length and its first bytes, for the fallback. */
    pub loader_size: u64,
    pub loader_head: &'a [u8],
}

pub fn is_boot_media(disk: &mut dyn BlockSink, ev: &BootEvidence<'_>) -> bool {
    if let Some(p) = &ev.partition {
        return table_names(p, disk);
    }
    if ev.loader_size == 0 || ev.loader_head.is_empty() {
        return false;
    }
    let starts = volume_starts(disk);
    starts.into_iter().any(|base| holds_loader(disk, base, ev.loader_size, ev.loader_head))
}
