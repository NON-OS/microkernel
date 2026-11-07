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

//! Reclaiming the volume's unreached blocks when it has no fresh ones left
//! (blockfs::reclaim). Asked by a write the volume refused for room, never
//! on a normal start, so a boot reads nothing more than it did.
//!
//! Not while an import is fed or paused: a fed stream's blocks are written
//! before any file names them, and a paused one's are named only inside its
//! mark, so the walk would take them for free. The caller asks `streaming`
//! before it takes the volume's hold; marks are looked for under the hold.

use super::error::VolumeError;
use super::import_guard::is_mark;
use super::state::VolumeState;
use crate::fs::blockfs::reclaim::{install, reach, Blocks, WalkError};
use crate::fs::blockfs::{self, BlockFsError, MODE_DIR};
use crate::fs::cryptoblock::{self, CryptoBlockError, PLAIN_BLOCK_BYTES};

struct Sealed<'a>(&'a [u8; 32]);

impl Blocks for Sealed<'_> {
    type Error = BlockFsError;

    fn node(&mut self, lba: u64) -> Result<(bool, u64), BlockFsError> {
        let n = blockfs::read_node(self.0, lba)?;
        Ok((n.mode & MODE_DIR != 0, n.first_record_lba))
    }

    fn block(&mut self, lba: u64) -> Result<[u8; PLAIN_BLOCK_BYTES], BlockFsError> {
        cryptoblock::read(self.0, lba).map_err(BlockFsError::CryptoBlock)
    }
}

/// Walks the tree and frees every block it does not reach. Returns how
/// many blocks were freed: 0 when a mark is on the volume.
pub(super) fn reclaim(state: &mut VolumeState) -> Result<u64, VolumeError> {
    let root = blockfs::read_root(&state.key, &state.mount).map_err(VolumeError::BlockFs)?;
    let names = blockfs::list(&state.key, &root).map_err(VolumeError::BlockFs)?;
    if names.iter().any(|e| is_mark(&e.name)) {
        return Ok(0);
    }
    // What a writer held is on the disk before anything is judged unreached.
    cryptoblock::flush_held()
        .map_err(|e: CryptoBlockError| VolumeError::BlockFs(BlockFsError::CryptoBlock(e)))?;
    let started = crate::sys::clock::since_boot_ms();
    let sb = &state.mount.superblock;
    let reached = match reach(
        &mut Sealed(&state.key),
        sb.root_lba,
        blockfs::HEADER_RING_SECTORS,
        sb.free_lba,
    ) {
        Ok(r) => r,
        Err(WalkError::Read(e)) => return Err(VolumeError::BlockFs(e)),
        Err(WalkError::Shape) => return Err(VolumeError::BlockFs(BlockFsError::InvalidRecord)),
    };
    let freed = install(&state.mount, &reached.runs);
    let took = crate::sys::clock::since_boot_ms().saturating_sub(started);
    super::say::say(&alloc::format!(
        "[DATA] reclaimed {} blocks ({} KiB) in {} ms: {} nodes, {} blocks read",
        freed,
        freed * PLAIN_BLOCK_BYTES as u64 / 1024,
        took,
        reached.nodes,
        reached.reads
    ));
    Ok(freed)
}

/// The fewest milliseconds between two reclaims. A walk reads about one
/// block in sixty of every file (the pointer blocks), so a volume full of
/// what is still named is not walked again for every write it refuses.
const APART_MS: u64 = 30_000;

static LAST_MS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

fn due() -> bool {
    let now = crate::sys::clock::since_boot_ms();
    let last = LAST_MS.load(core::sync::atomic::Ordering::Relaxed);
    (last == 0 || now.saturating_sub(last) >= APART_MS) && {
        LAST_MS.store(now.max(1), core::sync::atomic::Ordering::Relaxed);
        true
    }
}

/// Runs `write`; when the volume refuses it for room, no import is in
/// flight and no reclaim ran in the last APART_MS, reclaims and runs it
/// once more.
pub(super) fn with_room<T>(
    mut write: impl FnMut(&mut VolumeState) -> Result<T, VolumeError>,
) -> Result<T, VolumeError> {
    let first = {
        let mut guard = super::state::VOLUME.write();
        write(guard.as_mut().ok_or(VolumeError::NotMounted)?)
    };
    match first {
        Err(VolumeError::BlockFs(BlockFsError::OutOfSpace))
            if !super::import_feed::streaming() && due() =>
        {
            let mut guard = super::state::VOLUME.write();
            let state = guard.as_mut().ok_or(VolumeError::NotMounted)?;
            match reclaim(state)? {
                0 => Err(VolumeError::BlockFs(BlockFsError::OutOfSpace)),
                _ => write(state),
            }
        }
        other => other,
    }
}
