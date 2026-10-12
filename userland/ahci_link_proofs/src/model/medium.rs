// NONOS Operating System (AGPL-3.0-or-later)
//! The model disk's sectors. One never written holds its own LBA in every
//! eight bytes, so data read back says which sector it came from.

use std::collections::HashMap;

use crate::constants::ata::SECTOR_SIZE;

pub fn pattern(lba: u64) -> [u8; SECTOR_SIZE] {
    let mut s = [0u8; SECTOR_SIZE];
    for (i, word) in s.chunks_exact_mut(8).enumerate() {
        word.copy_from_slice(&(lba << 8 | i as u64).to_le_bytes());
    }
    s
}

#[derive(Default)]
pub struct Medium {
    written: HashMap<u64, [u8; SECTOR_SIZE]>,
}

impl Medium {
    pub fn sector(&self, lba: u64) -> [u8; SECTOR_SIZE] {
        self.written.get(&lba).copied().unwrap_or_else(|| pattern(lba))
    }

    /// `bytes` from sector `lba` on, into host memory at `to`.
    pub(super) fn read_into(&self, lba: u64, to: u64, bytes: u64) {
        let mut at = 0u64;
        while at < bytes {
            let n = (bytes - at).min(SECTOR_SIZE as u64);
            let s = self.sector(lba + at / SECTOR_SIZE as u64);
            // SAFETY: `to` is a buffer of the rig's, `bytes` long at least:
            // the PRD that named it holds no more.
            unsafe { core::ptr::copy_nonoverlapping(s.as_ptr(), (to + at) as *mut u8, n as usize) };
            at += n;
        }
    }

    /// `bytes` from host memory at `from` onto sector `lba` on. A partial
    /// last sector keeps the rest of what it held.
    pub(super) fn write_from(&mut self, lba: u64, from: u64, bytes: u64) {
        let mut at = 0u64;
        while at < bytes {
            let n = (bytes - at).min(SECTOR_SIZE as u64);
            let target = lba + at / SECTOR_SIZE as u64;
            let mut s = self.sector(target);
            // SAFETY: as in `read_into`.
            unsafe {
                core::ptr::copy_nonoverlapping((from + at) as *const u8, s.as_mut_ptr(), n as usize)
            };
            self.written.insert(target, s);
            at += n;
        }
    }
}
