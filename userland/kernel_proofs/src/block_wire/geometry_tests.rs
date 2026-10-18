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

use super::decode::decode;
use super::header::{GEOMETRY_LEN, MAGIC};

fn payload(count: u64, bytes: u32, max: u32, flags: u32) -> [u8; GEOMETRY_LEN] {
    let mut b = [0u8; GEOMETRY_LEN];
    b[..8].copy_from_slice(&count.to_le_bytes());
    b[8..12].copy_from_slice(&bytes.to_le_bytes());
    b[12..16].copy_from_slice(&max.to_le_bytes());
    b[16..20].copy_from_slice(&flags.to_le_bytes());
    b
}

#[test]
fn the_magic_spells_nbw1() {
    assert_eq!(&MAGIC.to_be_bytes(), b"NBW1");
}

#[test]
fn fields_come_back_little_endian_in_wire_order() {
    let g = decode(&payload(0x0123_4567_89ab, 4096, 65536, 0b101)).unwrap();
    assert_eq!(g.lba_count, 0x0123_4567_89ab);
    assert_eq!(g.lba_bytes, 4096);
    assert_eq!(g.max_transfer_bytes, 65536);
    assert!(g.removable() && !g.read_only() && g.media_present());
}

#[test]
fn each_flag_is_its_own_bit() {
    let ro = decode(&payload(1, 512, 512, 0b110)).unwrap();
    assert!(!ro.removable() && ro.read_only() && ro.media_present());
    let empty = decode(&payload(0, 512, 512, 0b001)).unwrap();
    assert!(empty.removable() && !empty.media_present());
}

#[test]
fn a_short_payload_is_refused_and_a_longer_one_read_by_its_head() {
    let full = payload(8, 512, 4096, 4);
    for n in 0..GEOMETRY_LEN {
        assert!(decode(&full[..n]).is_none(), "{n}");
    }
    let mut longer = full.to_vec();
    longer.extend_from_slice(&[0xff; 12]);
    assert_eq!(decode(&longer), decode(&full));
}
