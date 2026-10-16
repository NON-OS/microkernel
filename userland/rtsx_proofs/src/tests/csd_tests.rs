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

//! CSD decoding against the field layout of the SD Physical Layer
//! Specification (5.3.2 and 5.3.3), with UNSTUFF_BITS as Linux has it.

use crate::fields::put;
use crate::sd::bits::unstuff;
use crate::sd::capacity_blocks;

#[test]
fn unstuff_reads_fields_across_a_word_boundary() {
    let mut r = [0u32; 4];
    put(&mut r, 48, 22, 0x2A_BCDE);
    assert_eq!(unstuff(&r, 48, 22), 0x2A_BCDE);
    put(&mut r, 126, 2, 1);
    assert_eq!(unstuff(&r, 126, 2), 1);
    assert_eq!(r[0] >> 30, 1);
}

#[test]
fn a_version_2_csd_is_c_size_plus_one_times_512_kib() {
    // A 32 GB SDHC card: C_SIZE 60719 gives 60720 * 1024 blocks.
    let mut csd = [0u32; 4];
    put(&mut csd, 126, 2, 1);
    put(&mut csd, 48, 22, 60719);
    assert_eq!(capacity_blocks(&csd), Some(60720 * 1024));
}

#[test]
fn a_version_1_csd_uses_c_size_its_multiplier_and_block_length() {
    // 1 GB: C_SIZE 4095, C_SIZE_MULT 7, READ_BL_LEN 9 (512 bytes):
    // 4096 * 2^9 blocks of 512 bytes.
    let mut csd = [0u32; 4];
    put(&mut csd, 62, 12, 4095);
    put(&mut csd, 47, 3, 7);
    put(&mut csd, 80, 4, 9);
    assert_eq!(capacity_blocks(&csd), Some(4096 * 512));
    // 2 GB with READ_BL_LEN 10 (1024 bytes): still counted in 512-byte blocks.
    let mut big = [0u32; 4];
    put(&mut big, 62, 12, 4095);
    put(&mut big, 47, 3, 7);
    put(&mut big, 80, 4, 10);
    assert_eq!(capacity_blocks(&big), Some(4096 * 512 * 2));
}

#[test]
fn sduc_takes_a_28_bit_c_size_and_a_reserved_structure_is_refused() {
    let mut csd = [0u32; 4];
    put(&mut csd, 126, 2, 2);
    put(&mut csd, 48, 28, 0x0FFF_FFFF);
    assert_eq!(capacity_blocks(&csd), Some(0x1000_0000 << 10));
    let mut bad = [0u32; 4];
    put(&mut bad, 126, 2, 3);
    assert_eq!(capacity_blocks(&bad), None);
}
