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

//! What the driver serves from a drive's IDENTIFY DEVICE block. The block is
//! the drive's own 512 bytes, so these hold the driver's rule over the shapes
//! real drives send and over the named edges of each word it reads.

use crate::constants::identify::IDENTIFY_WORDS;
use crate::identity::{capacity, Refusal};
use crate::protocol::E_NXIO;
use crate::rw_parse::parse;

pub(crate) type Block = [u16; IDENTIFY_WORDS];

pub(crate) const LBA48_LIMIT: u64 = 1 << 48;

/// The words QEMU's ide_identify and ide_identify_size fill for a disk of
/// `sectors` 512-byte sectors (hw/ide/core.c), the shape a q35 boot answers.
pub(crate) fn qemu_block(sectors: u64) -> Block {
    let mut w = [0u16; IDENTIFY_WORDS];
    w[0] = 0x0040;
    let lba28 = sectors.min((1 << 28) - 1);
    w[60] = lba28 as u16;
    w[61] = (lba28 >> 16) as u16;
    w[83] = (1 << 14) | (1 << 13) | (1 << 12) | (1 << 10);
    w[86] = (1 << 13) | (1 << 12) | (1 << 10);
    w[87] = 1 << 14;
    set_lba48(&mut w, sectors);
    w[106] = 0x6000;
    w
}

pub(crate) fn set_lba48(w: &mut Block, sectors: u64) {
    w[100] = sectors as u16;
    w[101] = (sectors >> 16) as u16;
    w[102] = (sectors >> 32) as u16;
    w[103] = (sectors >> 48) as u16;
}

/// Word 106 valid and saying the logical sector is longer than 256 words,
/// with words 117-118 giving its length as `length_words`.
pub(crate) fn set_long_sector(w: &mut Block, length_words: u32) {
    w[106] = 0x5000;
    w[117] = length_words as u16;
    w[118] = (length_words >> 16) as u16;
}

fn set_lba28(w: &mut Block, sectors: u32) {
    w[60] = sectors as u16;
    w[61] = (sectors >> 16) as u16;
}

fn rw_body(lba: u64, n: u32) -> [u8; 12] {
    let mut b = [0u8; 12];
    b[0..8].copy_from_slice(&lba.to_le_bytes());
    b[8..12].copy_from_slice(&n.to_le_bytes());
    b
}

#[test]
fn a_qemu_disk_serves_its_sector_count() {
    assert_eq!(capacity(&qemu_block(131_072)), Ok(131_072), "the 64 MiB q35 data disk");
    assert_eq!(capacity(&qemu_block(1)), Ok(1));
}

#[test]
fn a_disk_past_28_bits_serves_its_48_bit_count() {
    let sectors = (1u64 << 32) + 5;
    let w = qemu_block(sectors);
    assert_eq!((u64::from(w[61]) << 16) | u64::from(w[60]), 0x0FFF_FFFF, "words 60-61 saturate");
    assert_eq!(capacity(&w), Ok(sectors));
}

#[test]
fn the_48_bit_count_is_served_up_to_the_last_lba_a_fis_can_name() {
    assert_eq!(capacity(&qemu_block(LBA48_LIMIT - 1)), Ok(LBA48_LIMIT - 1));
    assert_eq!(capacity(&qemu_block(LBA48_LIMIT)), Err(Refusal::PastLba48));
    assert_eq!(capacity(&qemu_block(LBA48_LIMIT + 1)), Err(Refusal::PastLba48));
    assert_eq!(capacity(&qemu_block(u64::MAX)), Err(Refusal::PastLba48));
    for bit in 48..64 {
        assert_eq!(capacity(&qemu_block(1 << bit)), Err(Refusal::PastLba48), "bit {bit}");
    }
}

#[test]
fn a_zero_48_bit_count_refuses_the_disk_whatever_words_60_61_say() {
    let mut w = qemu_block(0);
    for lba28 in [0u32, 1, 1000, 0x0FFF_FFFF, 0x1000_0000, u32::MAX] {
        set_lba28(&mut w, lba28);
        assert_eq!(capacity(&w), Err(Refusal::NoCapacity), "words 60-61 = {lba28:#x}");
    }
}

#[test]
fn a_28_bit_only_disk_is_refused_however_sane_its_count() {
    /*
     * A disk without the 48-bit feature set aborts READ DMA EXT, the only
     * read the driver issues, so even a well-formed 28-bit count is not
     * served, and an LBA28 count past 0x0FFFFFFF certainly is not.
     */
    for lba28 in [1u32, 1000, 0x0FFF_FFFF, 0x1000_0000, u32::MAX] {
        let mut w = qemu_block(0);
        w[83] &= !(1 << 10);
        w[86] &= !(1 << 10);
        set_lba28(&mut w, lba28);
        assert_eq!(capacity(&w), Err(Refusal::No48BitAddress), "words 60-61 = {lba28:#x}");
    }
}

#[test]
fn a_48_bit_count_without_both_feature_bits_is_refused() {
    let mut supported_only = qemu_block(1000);
    supported_only[86] &= !(1 << 10);
    assert_eq!(capacity(&supported_only), Err(Refusal::No48BitAddress), "supported, not enabled");

    let mut enabled_only = qemu_block(1000);
    enabled_only[83] &= !(1 << 10);
    assert_eq!(capacity(&enabled_only), Err(Refusal::No48BitAddress), "enabled, not supported");
}

#[test]
fn feature_words_count_only_under_their_validity_bits() {
    for top in [0x0000u16, 0x8000, 0xc000] {
        let mut w = qemu_block(1000);
        w[83] = (w[83] & !0xc000) | top;
        assert_eq!(capacity(&w), Err(Refusal::No48BitAddress), "word 83 bits 15:14 = {top:#x}");

        let mut w = qemu_block(1000);
        w[87] = (w[87] & !0xc000) | top;
        assert_eq!(capacity(&w), Err(Refusal::No48BitAddress), "word 87 bits 15:14 = {top:#x}");
    }
}

#[test]
fn an_all_zero_or_all_ones_block_is_refused() {
    assert_eq!(capacity(&[0u16; IDENTIFY_WORDS]), Err(Refusal::No48BitAddress));
    assert_eq!(capacity(&[0xffffu16; IDENTIFY_WORDS]), Err(Refusal::No48BitAddress));
}

#[test]
fn words_60_61_never_change_what_a_48_bit_disk_serves() {
    for lba28 in [0u32, 1, 999, 1001, 0x0FFF_FFFF, 0x1000_0000, u32::MAX] {
        let mut w = qemu_block(1000);
        set_lba28(&mut w, lba28);
        assert_eq!(capacity(&w), Ok(1000), "words 60-61 = {lba28:#x}");
    }
}

#[test]
fn every_request_on_the_largest_served_disk_names_an_lba_a_fis_carries() {
    let served = capacity(&qemu_block(LBA48_LIMIT - 1)).expect("largest 48-bit disk is served");
    assert_eq!(parse(&rw_body(served - 1, 1), served), Ok((served - 1, 1)), "the last sector");
    assert_eq!(parse(&rw_body(served, 1), served), Err(E_NXIO), "the first LBA past 48 bits");
    assert_eq!(parse(&rw_body(LBA48_LIMIT + 5, 1), served), Err(E_NXIO), "an LBA a FIS would wrap");
}

#[test]
fn word_106_naming_no_long_sector_means_512_byte_sectors() {
    /*
     * Valid with bit 12 clear (QEMU's 0x6000 among them), or not valid at
     * all because bits 15:14 read anything but 01b: either way the sector is
     * 256 words, and words 117-118, set here to a 4096-byte figure, are not
     * read.
     */
    for w106 in [0x6000u16, 0x4000, 0x6003, 0x0000, 0x1000, 0x9000, 0xd000, 0xffff] {
        let mut w = qemu_block(1000);
        w[106] = w106;
        w[117] = 2048;
        assert_eq!(capacity(&w), Ok(1000), "word 106 = {w106:#x}");
    }
}

#[test]
fn a_long_sector_whose_length_is_256_words_is_512_bytes() {
    let mut w = qemu_block(1000);
    set_long_sector(&mut w, 256);
    assert_eq!(capacity(&w), Ok(1000));
}

#[test]
fn any_logical_sector_but_512_bytes_refuses_the_disk() {
    let named = [
        (0u32, "no length"),
        (1, "one word"),
        (255, "510 bytes"),
        (257, "514 bytes"),
        (260, "520-byte fat sectors"),
        (264, "528-byte fat sectors"),
        (1024, "2048 bytes"),
        (2048, "a 4096-byte native disk, sane but not served"),
        (4096, "8192 bytes, past the 4096 limit"),
        (0xffff, "the most word 117 holds"),
        (0x1_0000, "word 118 alone"),
        (0x8000_0100, "a length whose doubling wraps 32 bits to 512"),
        (0x8000_0000, "a length whose doubling wraps 32 bits to 0"),
        (u32::MAX, "every bit set"),
    ];
    for (length_words, what) in named {
        let mut w = qemu_block(1000);
        set_long_sector(&mut w, length_words);
        assert_eq!(capacity(&w), Err(Refusal::SectorSize), "{what}");
    }
}
