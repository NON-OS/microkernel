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

//! An IVRS walk says when it left an IOMMU unread (K11): past the eight
//! the kernel holds, or past a block that does not parse.

use super::ivrs_walk::{walk, MAX_AMD_IOMMUS};

/// An IVRS of 24-byte IVHD blocks at these bases, then `tail`.
fn table(bases: impl IntoIterator<Item = u64>, tail: &[u8]) -> Vec<u8> {
    let mut t = vec![0u8; 48];
    for base in bases {
        let mut b = [0u8; 24];
        b[0] = 0x10;
        b[2] = 24;
        b[8..16].copy_from_slice(&base.to_le_bytes());
        t.extend_from_slice(&b);
    }
    t.extend_from_slice(tail);
    t
}

fn read(t: &[u8]) -> (usize, bool) {
    let w = walk(t, &mut [0u64; MAX_AMD_IOMMUS]);
    (w.found, w.complete)
}

#[test]
fn a_whole_table_is_read_whole() {
    assert_eq!(read(&table([0xfd20_0000, 0xfd30_0000], &[])), (2, true));
    assert_eq!(read(&table([0xfd20_0000; 3], &[])), (1, true), "one unit thrice");
    assert_eq!(read(&[]), (0, true));
}

#[test]
fn a_ninth_iommu_is_left_off_and_said_so() {
    let eight = (1..=8).map(|i| i * 0x1000);
    assert_eq!(read(&table(eight, &[])), (MAX_AMD_IOMMUS, true));
    let nine = (1..=9).map(|i| i * 0x1000);
    assert_eq!(read(&table(nine, &[])), (MAX_AMD_IOMMUS, false));
}

#[test]
fn a_block_that_does_not_parse_leaves_the_rest_unread() {
    let runs_past = table([0xfd20_0000], &[0x10, 0, 0xFF, 0xFF]);
    assert_eq!(read(&runs_past), (1, false));
    assert_eq!(read(&table([], &[0x10, 0, 2, 0])), (0, false), "length under 4");
}
