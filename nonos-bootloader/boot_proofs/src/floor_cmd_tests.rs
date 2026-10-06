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

//! The counter's commands, and the read's mapping byte by byte (REVIEW R20).

use crate::security::tpm_nv::floor_cmd::{
    base_define, base_lock, base_read, base_write, floor_read, rb_define, rb_increment, rb_read, FloorRead,
};

#[test]
fn the_commands_name_the_rollback_counter() {
    let (d, i, r) = (rb_define(), rb_increment(), rb_read());
    assert_eq!(d[31..35], [0x01, 0x00, 0x00, 0x20], "define publicInfo.nvIndex");
    assert_eq!(d[37..41], [0x02, 0x04, 0x00, 0x14], "counter, auth read and write, no DA");
    for c in [&i[..], &r[..]] {
        assert_eq!((&c[10..14], &c[14..18]), (&[1, 0, 0, 0x20][..], &[1, 0, 0, 0x20][..]));
    }
    for c in [&d[..], &i[..], &r[..]] {
        assert_eq!(u32::from_be_bytes(c[2..6].try_into().unwrap()) as usize, c.len());
    }
}

#[test]
fn the_read_is_a_value_only_in_the_shape_of_one() {
    let mut ok = [0u8; 29];
    ok[..10].copy_from_slice(&[0x80, 0x02, 0, 0, 0, 29, 0, 0, 0, 0]);
    ok[10..24].copy_from_slice(&[0, 0, 0, 10, 0, 8, 0, 0, 0, 0, 0, 0, 1, 2]);
    assert_eq!(floor_read(&ok, 29), FloorRead::Value(0x0102));
    assert_eq!(floor_read(&ok, 24), FloorRead::Value(0x0102));
    for (at, n) in [(13, 29), (15, 29)] {
        let mut bad = ok;
        bad[at] ^= 1;
        assert_eq!(floor_read(&bad, n), FloorRead::Unreadable, "byte {at}");
    }
    for n in [0, 9, 10, 23, 30] {
        assert_eq!(floor_read(&ok, n), FloorRead::Unreadable, "{n} bytes");
    }
    let mut un = ok;
    un[6..10].copy_from_slice(&0x14Au32.to_be_bytes());
    assert_eq!(floor_read(&un, 10), FloorRead::Uninitialized);
}

#[test]
fn the_base_commands_name_the_base_and_lock_it() {
    let (d, r, w, l) = (base_define(), base_read(), base_write(0x0102_0304), base_lock());
    assert_eq!(d[31..35], [0x01, 0x00, 0x00, 0x21], "define publicInfo.nvIndex");
    assert_eq!(d[37..41], [0x02, 0x04, 0x20, 0x04], "ordinary, auth read and write, write define, no DA");
    assert_eq!(u32::from_be_bytes(w[6..10].try_into().unwrap()), 0x137, "TPM2_NV_Write");
    assert_eq!(u32::from_be_bytes(l[6..10].try_into().unwrap()), 0x138, "TPM2_NV_WriteLock");
    for c in [&r[..], &w[..], &l[..]] {
        assert_eq!((&c[10..14], &c[14..18]), (&[1, 0, 0, 0x21][..], &[1, 0, 0, 0x21][..]));
        assert_eq!(&c[18..26], &[0, 0, 0, 9, 0x40, 0, 0, 9], "an empty password session");
    }
    assert_eq!(&w[31..43], &[0, 8, 0, 0, 0, 0, 1, 2, 3, 4, 0, 0], "eight bytes at offset 0");
    for c in [&d[..], &r[..], &w[..], &l[..]] {
        assert_eq!(u32::from_be_bytes(c[2..6].try_into().unwrap()) as usize, c.len());
    }
}
