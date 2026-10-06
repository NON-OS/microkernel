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

use super::dte::{allows_write, blocked, domain_of, mode_of, passthrough, root_of, translated};
use super::pte::{
    address, directory, index, is_present, leaf, next_level, reach, writable, LEVELS,
};

/* Linux init_device_table_dma (V and TV only) and set_dte_entry (root, mode
in bits 11:9, IR bit 61, IW bit 62, domain id in the second quadword). */
#[test]
fn device_table_entries_match_linux() {
    assert_eq!(blocked(), [0x3, 0, 0, 0]);
    assert!(!allows_write(blocked()));
    assert_eq!(passthrough()[0], 0x6000_0000_0000_0003);
    assert_eq!(mode_of(passthrough()), 0);
    let t = translated(0x1234_5000, 4, 7);
    assert_eq!(t[0], 0x6000_0000_1234_5803);
    assert_eq!((domain_of(t), mode_of(t), root_of(t)), (7, 4, 0x1234_5000));
    assert_eq!(root_of(translated(0x1234_5FFF, 4, 7)), 0x1234_5000);
}

/* Linux PM_LEVEL_PDE: a directory names the level below it in bits 11:9 with
PR, IR and IW; a 4 KiB leaf is PR | FC | address with IR and IW as asked. */
#[test]
fn page_table_entries_match_linux() {
    let top = directory(0x5000, 4);
    assert_eq!(top, 1 | (3 << 9) | 0x5000 | (1 << 61) | (1 << 62));
    assert_eq!((next_level(top), address(top)), (3, 0x5000));
    assert_eq!(next_level(directory(0x6000, 2)), 1);
    let ro = leaf(0x7000, true, false);
    assert_eq!(ro, 1 | (1 << 60) | 0x7000 | (1 << 61));
    assert!(is_present(ro) && !writable(ro) && next_level(ro) == 0);
    assert!(writable(leaf(0x7000, true, true)));
}

#[test]
fn four_levels_index_every_bit_of_a_48_bit_iova() {
    assert_eq!(reach(LEVELS), 1 << 48);
    let iova = (0x100u64 << 39) | (0x101 << 30) | (0x080 << 21) | (0x101 << 12) | 0xABC;
    assert_eq!(index(iova, 4), 0x100);
    assert_eq!(index(iova, 3), 0x101);
    assert_eq!(index(iova, 2), 0x080);
    assert_eq!(index(iova, 1), 0x101);
    for level in 1..=LEVELS {
        assert_eq!(index(u64::MAX, level), 0x1FF);
    }
}
