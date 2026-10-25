// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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
use super::constants::HEADER_RING_SECTORS;
use super::read_header::read_header;
use super::ring_pick::{pick, SlotRead};
use super::{BlockFsError, BlockFsMount};
use crate::fs::cryptoblock::CryptoBlockError;

pub fn mount(key: &[u8; 32]) -> Result<BlockFsMount, BlockFsError> {
    let slots = (0..HEADER_RING_SECTORS).map(|lba| match read_header(key, lba) {
        Ok(m) => SlotRead::Header { at: lba, generation: m.superblock.generation, header: m },
        Err(e) if not_ours(&e) => SlotRead::NotOurs,
        Err(e) => SlotRead::Failed(e),
    });
    pick(slots, HEADER_RING_SECTORS)?.ok_or(BlockFsError::NotFormatted)
}

/// A slot that does not open under the key, or opens to something that is
/// not a header, holds no header of this volume; every other error is the
/// device's.
fn not_ours(e: &BlockFsError) -> bool {
    matches!(
        e,
        BlockFsError::CryptoBlock(CryptoBlockError::AuthenticationFailed)
            | BlockFsError::InvalidSuperblock
    )
}
