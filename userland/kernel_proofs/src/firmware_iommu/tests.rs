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

use super::ivrs_walk::MAX_AMD_IOMMUS;

fn iommu_bases(table: &[u8]) -> Vec<u64> {
    let mut out = [0u64; MAX_AMD_IOMMUS];
    let n = super::ivrs_walk::walk(table, &mut out).found;
    out[..n].to_vec()
}

fn ivrs(blocks: &[(u8, u16, u64)]) -> Vec<u8> {
    let mut t = vec![0u8; 48];
    t[0..4].copy_from_slice(b"IVRS");
    for (kind, len, base) in blocks {
        let mut b = vec![0u8; (*len as usize).max(4)];
        b[0] = *kind;
        b[2..4].copy_from_slice(&len.to_le_bytes());
        if *len >= 16 {
            b[8..16].copy_from_slice(&base.to_le_bytes());
        }
        t.extend_from_slice(&b);
    }
    t
}

#[test]
fn one_iommu_described_twice_is_one_base() {
    let t = ivrs(&[(0x10, 24, 0xfd20_0000), (0x11, 40, 0xfd20_0000)]);
    assert_eq!(iommu_bases(&t).as_slice(), &[0xfd20_0000]);
}

#[test]
fn every_unit_is_found_and_other_blocks_skipped() {
    let t = ivrs(&[(0x10, 24, 0xfd20_0000), (0x20, 32, 0xdead), (0x40, 40, 0xfd30_0000)]);
    assert_eq!(iommu_bases(&t).as_slice(), &[0xfd20_0000, 0xfd30_0000]);
}

#[test]
fn hostile_tables_end_the_walk_without_reading_past_them() {
    assert!(iommu_bases(&[]).is_empty());
    assert!(iommu_bases(&[0u8; 47]).is_empty());
    let mut t = ivrs(&[(0x10, 24, 0xfd20_0000)]);
    t.extend_from_slice(&[0x10, 0, 0xFF, 0xFF]);
    assert_eq!(iommu_bases(&t).as_slice(), &[0xfd20_0000]);
    let zero_len = ivrs(&[(0x10, 0, 0)]);
    assert!(iommu_bases(&zero_len).is_empty());
    let short = ivrs(&[(0x10, 12, 0)]);
    assert!(iommu_bases(&short).is_empty());
    let many: Vec<_> = (0..20u64).map(|i| (0x10u8, 24u16, 0x1000 * (i + 1))).collect();
    assert_eq!(iommu_bases(&ivrs(&many)).len(), MAX_AMD_IOMMUS);
}
