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

//! READ CAPACITY (10) and (16) (SBC-4, 5.20 and 5.21).

pub const RC10_LEN: usize = 8;
pub const RC16_LEN: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capacity {
    /// The number of blocks and their size.
    Known { blocks: u64, block_bytes: u32 },
    /// The last block is past what READ CAPACITY (10) can name: ask (16).
    TooLarge,
}

pub fn rc10(b: &[u8]) -> Option<Capacity> {
    let b = b.get(..RC10_LEN)?;
    let last = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
    let block_bytes = u32::from_be_bytes([b[4], b[5], b[6], b[7]]);
    if last == u32::MAX {
        return Some(Capacity::TooLarge);
    }
    Some(Capacity::Known { blocks: last as u64 + 1, block_bytes })
}

pub fn rc16(b: &[u8]) -> Option<Capacity> {
    let b = b.get(..12)?;
    let last = u64::from_be_bytes(b[0..8].try_into().ok()?);
    let block_bytes = u32::from_be_bytes(b[8..12].try_into().ok()?);
    Some(Capacity::Known { blocks: last.checked_add(1)?, block_bytes })
}

/// A block size the layer serves: a power of two from 512 bytes to 4 KiB.
pub const fn usable_block(bytes: u32) -> bool {
    bytes.is_power_of_two() && bytes >= 512 && bytes <= 4096
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_and_sixteen() {
        assert_eq!(
            rc10(&[0, 0, 0x0F, 0xFF, 0, 0, 2, 0]),
            Some(Capacity::Known { blocks: 4096, block_bytes: 512 })
        );
        assert_eq!(rc10(&[0xFF; 8]), Some(Capacity::TooLarge));
        let mut b = [0u8; 32];
        b[0..8].copy_from_slice(&0x1_2345_6789u64.to_be_bytes());
        b[8..12].copy_from_slice(&4096u32.to_be_bytes());
        assert_eq!(rc16(&b), Some(Capacity::Known { blocks: 0x1_2345_678A, block_bytes: 4096 }));
        assert_eq!(rc16(&[0xFF; 12]), None, "a last block of u64::MAX counts no blocks");
        assert_eq!(rc10(&[0; 7]), None);
    }

    #[test]
    fn block_sizes() {
        assert!(usable_block(512) && usable_block(2048) && usable_block(4096));
        assert!(
            !usable_block(520) && !usable_block(256) && !usable_block(8192) && !usable_block(0)
        );
    }
}
