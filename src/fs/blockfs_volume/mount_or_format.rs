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

//! Mounting the volume under a key, formatting it only over a blank ring.

use super::error::VolumeError;
use super::key_header::Keyed;
use super::key_header_io::write_key_header;
use super::plan_types::Plan;
use super::ring_blank::ring_blank;
use super::say::say;
use crate::fs::blockfs::{self, BlockFsError, BlockFsMount};
use alloc::format;

/// Say when this boot's plan opens a window other than the one the volume
/// was formatted to. More room is grown into; less leaves every block past
/// it unreadable, which is said here rather than met later as a failed read.
fn window_check(m: &BlockFsMount, plan: &Plan) {
    let (formatted, window, used) = (m.superblock.sectors, plan.volume_sectors, m.superblock.free_lba);
    if window == formatted {
        return;
    }
    if used > window {
        crate::log::warn!(
            "[DATA] the plan opens {} sectors but the volume, formatted at {}, uses {}; \
             blocks past the window cannot be read. Plan it again with at least {} sectors",
            window,
            formatted,
            used,
            used
        );
    } else {
        say(&format!("[DATA] the volume, formatted at {formatted} sectors, has {window} this boot"));
    }
}

/// Mount the volume the plan names under `key`. Only a header ring that has
/// never been written is formatted, after `keyed` is recorded: a ring that
/// holds sectors this key cannot open is someone's data under another key,
/// and is left alone.
pub(super) fn mount_or_format(
    key: &[u8; 32],
    plan: &Plan,
    keyed: &Keyed,
) -> Result<BlockFsMount, VolumeError> {
    match blockfs::mount(key) {
        Ok(m) => {
            window_check(&m, plan);
            Ok(m)
        }
        Err(BlockFsError::NotFormatted) if ring_blank(plan.volume_base)? => {
            write_key_header(keyed)?;
            let mut uuid = [0u8; 16];
            crate::crypto::rng::fill_random_bytes(&mut uuid);
            let m = blockfs::format(key, uuid).map_err(VolumeError::BlockFs)?;
            say(&format!("[DATA] formatted a volume of {} sectors", plan.volume_sectors));
            Ok(m)
        }
        Err(BlockFsError::NotFormatted) => {
            crate::log::warn!(
                "[DATA] the volume holds data this key cannot open; not formatting over it"
            );
            Err(VolumeError::Unopenable)
        }
        Err(e) => Err(VolumeError::BlockFs(e)),
    }
}
