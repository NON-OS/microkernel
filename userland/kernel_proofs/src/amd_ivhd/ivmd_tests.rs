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

//! K4: IVMD ranges, the memory an unclaimed AMD device keeps one to one,
//! per spec 48882 5.2.2.3, from well formed and hostile tables.

use super::ivmd::{reserved, Reserved, MAX_RESERVED};

fn ivmd(kind: u8, flags: u8, (id, aux): (u16, u16), start: u64, len: u64) -> Vec<u8> {
    let mut b = vec![kind, flags, 32, 0];
    b.extend_from_slice(&id.to_le_bytes());
    b.extend_from_slice(&aux.to_le_bytes());
    b.extend_from_slice(&[0; 8]);
    b.extend_from_slice(&start.to_le_bytes());
    b.extend_from_slice(&len.to_le_bytes());
    b
}

fn read(blocks: &[Vec<u8>]) -> Vec<Reserved> {
    let mut t = vec![0u8; 48];
    blocks.iter().for_each(|b| t.extend_from_slice(b));
    let none = Reserved { first: 0, last: 0, start: 0, end: 0, read: false, write: false };
    let mut out = [none; MAX_RESERVED];
    let n = reserved(&t, &mut out);
    out[..n].to_vec()
}

#[test]
fn each_kind_names_its_requester_ids() {
    let got = read(&[
        ivmd(0x20, 0b011, (0, 0), 0xD000_0000, 0x1000),
        ivmd(0x21, 0b101, (0x00A0, 0), 0xE000_0000, 0x2000),
        ivmd(0x22, 0b111, (0x0100, 0x01FF), 0xF000_0000, 0x1000),
    ]);
    assert_eq!((got[0].first, got[0].last, got[0].read, got[0].write), (0, 0xFFFF, true, false));
    assert_eq!((got[1].first, got[1].last, got[1].end), (0xA0, 0xA0, 0xE000_2000));
    assert_eq!((got[2].first, got[2].last, got[2].write), (0x100, 0x1FF, true));
}

#[test]
fn ranges_round_out_to_pages_and_exclusion_is_read_write() {
    let got = read(&[ivmd(0x21, 0b1000, (1, 0), 0x1234_5678, 0x10)]);
    assert_eq!((got[0].start, got[0].end), (0x1234_5000, 0x1234_6000));
    assert!(got[0].read && got[0].write, "an exclusion range, as Linux since 5.5");
}

#[test]
fn hostile_blocks_name_nothing() {
    let no_unity = ivmd(0x21, 0b110, (1, 0), 0x1000, 0x1000);
    let no_rights = ivmd(0x21, 0b001, (1, 0), 0x1000, 0x1000);
    let empty = ivmd(0x21, 0b011, (1, 0), 0x1000, 0);
    let wraps = ivmd(0x21, 0b011, (1, 0), u64::MAX - 0xFFF, 0x2000);
    let backwards = ivmd(0x22, 0b011, (0x200, 0x100), 0x1000, 0x1000);
    assert!(read(&[no_unity, no_rights, empty, wraps, backwards]).is_empty());
    let short = vec![0x21, 0b011, 16, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(read(&[short]).is_empty(), "a block too short for its fields");
    let mut past = ivmd(0x21, 0b011, (1, 0), 0x1000, 0x1000);
    past[2] = 0xFF;
    assert!(read(&[past]).is_empty(), "a length past the table");
    let many: Vec<Vec<u8>> = (0..40).map(|i| ivmd(0x21, 0b011, (i, 0), 0x1000, 0x1000)).collect();
    assert_eq!(read(&many).len(), MAX_RESERVED);
}
