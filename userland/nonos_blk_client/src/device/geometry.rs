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

//! A disk's block size and per-request ceiling, as `span` cuts requests by.

use super::transfer::{max_transfer_bytes, DATA_BYTES, SECTOR};

/// A disk as its driver addresses it: the block size, and the most bytes
/// one request may move, a whole number of blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Geometry {
    lba_size: u32,
    max_bytes: u32,
}

impl Geometry {
    /// 512-byte blocks, sixty-four to a request: AHCI and virtio-blk.
    pub const SECTORS: Geometry = Geometry { lba_size: SECTOR as u32, max_bytes: DATA_BYTES };

    /// An NVMe namespace with blocks of `lba_size` bytes on a controller
    /// whose identify page gives `mdts`. The driver moves at most its data
    /// buffer per command, and less when MDTS says so; this mirrors the
    /// driver's own rule, so no request is one it refuses for its size.
    /// `None` for a block size that is not a power of two from 512 bytes to
    /// the buffer, or a transfer limit below one block.
    pub fn nvme(lba_size: u32, mdts: u8) -> Option<Geometry> {
        Self::sized(lba_size, max_transfer_bytes(mdts))
    }

    /// A block-wire disk (driver.sdhci0): blocks of `lba_size` bytes, at
    /// most `max_transfer` to a request as its GEOMETRY reply says, and
    /// never more than the buffer here. `None` on the same terms as `nvme`.
    pub fn wire(lba_size: u32, max_transfer: u32) -> Option<Geometry> {
        Self::sized(lba_size, max_transfer.min(DATA_BYTES))
    }

    fn sized(lba_size: u32, max: u32) -> Option<Geometry> {
        let fits = (SECTOR as u32..=DATA_BYTES.min(max)).contains(&lba_size);
        // Lazily: `max % lba_size` must not run for a zero block size.
        (fits && lba_size.is_power_of_two())
            .then(|| Geometry { lba_size, max_bytes: max - max % lba_size })
    }

    pub fn lba_size(&self) -> u32 {
        self.lba_size
    }

    pub fn max_bytes(&self) -> u32 {
        self.max_bytes
    }

    /// Sectors in one block.
    pub fn per_block(&self) -> u64 {
        u64::from(self.lba_size) / SECTOR as u64
    }

    /// A capacity in blocks, as sectors. Saturated: a count that wrapped
    /// would describe a disk as a few bytes.
    pub fn sectors(&self, blocks: u64) -> u64 {
        blocks.saturating_mul(self.per_block())
    }
}
