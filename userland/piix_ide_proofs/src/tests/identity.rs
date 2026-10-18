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

//! IDENTIFY DEVICE read for what the driver needs.

use crate::disk::{parse, Flush};
use crate::model::{put_text, Drive};

fn words(d: &Drive) -> [u16; 256] {
    d.identify
}

#[test]
fn lba48_capacity_comes_from_words_100_to_103() {
    let id = parse(&words(&Drive::disk(0x1_2345_6789, true))).unwrap();
    assert_eq!(id.sectors, 0x1_2345_6789);
    assert!(id.lba48);
    assert_eq!(id.flush, Flush::CacheExt);
}

#[test]
fn without_lba48_capacity_comes_from_words_60_and_61() {
    let id = parse(&words(&Drive::disk(1_000_000, false))).unwrap();
    assert_eq!(id.sectors, 1_000_000);
    assert!(!id.lba48);
    assert_eq!(id.flush, Flush::Cache);
}

#[test]
fn the_model_reads_high_byte_first() {
    let mut w = words(&Drive::disk(64, true));
    put_text(&mut w[27..47], b"VBOX HARDDISK                           ");
    let id = parse(&w).unwrap();
    assert_eq!(&id.model[..13], b"VBOX HARDDISK");
}

#[test]
fn a_disk_with_neither_cache_word_has_nothing_to_flush() {
    let mut w = words(&Drive::disk(64, false));
    w[82] = 0x4000;
    w[83] = 0x4000;
    assert_eq!(parse(&w).unwrap().flush, Flush::None);
}

#[test]
fn command_set_words_without_their_valid_mark_are_ignored() {
    let mut w = words(&Drive::disk(64, true));
    w[83] = (1 << 10) | (1 << 13);
    w[82] = 0;
    let id = parse(&w).unwrap();
    assert!(!id.lba48, "word 83 is not valid without bit 14");
    assert_eq!(id.sectors, 64);
}

#[test]
fn refusals() {
    let base = words(&Drive::disk(64, true));
    let mut w = base;
    w[0] |= 1 << 15;
    assert_eq!(parse(&w).err(), Some("not an ATA disk"));
    let mut w = base;
    w[49] &= !(1 << 9);
    assert_eq!(parse(&w).err(), Some("no LBA addressing"));
    let mut w = base;
    w[49] &= !(1 << 8);
    assert_eq!(parse(&w).err(), Some("no DMA"));
    let mut w = base;
    w[106] = 0x4000 | (1 << 12);
    w[117] = 2048;
    assert_eq!(parse(&w).err(), Some("logical sectors are not 512 bytes"));
    w[117] = 256;
    assert!(parse(&w).is_ok(), "a long-sector word naming 512 bytes is fine");
    let mut w = base;
    w[100..104].fill(0);
    assert_eq!(parse(&w).err(), Some("no usable capacity"));
}
