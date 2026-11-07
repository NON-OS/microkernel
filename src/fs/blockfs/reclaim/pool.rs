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

//! The blocks a reclaim found free, handed out before the bump pointer.

use alloc::vec::Vec;
use spin::Mutex;

use super::super::alloc_block::alloc_limit;
use super::super::constants::FIRST_ALLOC_LBA;
use super::super::BlockFsMount;
use super::runs::gaps;

struct Pool {
    /// The volume these blocks are free on.
    uuid: [u8; 16],
    /// Free runs, highest last, so a run is taken from its end.
    runs: Vec<(u64, u64)>,
}

static POOL: Mutex<Option<Pool>> = Mutex::new(None);

/// Everything in [FIRST_ALLOC_LBA, free_lba) that `reached` does not cover
/// is free on `mount`'s volume from now on. Returns how many blocks.
pub(crate) fn install(mount: &BlockFsMount, reached: &[(u64, u64)]) -> u64 {
    let runs = gaps(FIRST_ALLOC_LBA, mount.superblock.free_lba, reached);
    let blocks = runs.iter().map(|(s, e)| e - s).sum();
    *POOL.lock() = Some(Pool { uuid: mount.superblock.uuid, runs });
    blocks
}

/// A free block of `mount`'s volume, when a reclaim found one.
pub(crate) fn take(mount: &BlockFsMount) -> Option<u64> {
    let mut pool = POOL.lock();
    let p = pool.as_mut().filter(|p| p.uuid == mount.superblock.uuid)?;
    let limit = alloc_limit(mount);
    while let Some(run) = p.runs.last_mut() {
        // A window made smaller since leaves the blocks past it out.
        if run.0 >= limit || run.0 >= run.1 {
            p.runs.pop();
            continue;
        }
        let lba = run.0;
        run.0 += 1;
        return Some(lba);
    }
    None
}

/// How many blocks a reclaim left free on `mount`'s volume.
pub(crate) fn room(mount: &BlockFsMount) -> u64 {
    let pool = POOL.lock();
    let p = pool.as_ref().filter(|p| p.uuid == mount.superblock.uuid);
    p.map_or(0, |p| p.runs.iter().map(|(s, e)| e.saturating_sub(*s)).sum())
}
