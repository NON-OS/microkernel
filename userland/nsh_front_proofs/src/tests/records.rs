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

//! `OP_RECORDS`'s frame: four length-prefixed records, each within its cap,
//! nothing after the last, and a frame that is not whole does not split.

use nsh_front_wire::{
    records_len, split_records, write_records, Records, ANCHOR_LEN, ENROL_MAX, LIST_MAX, TABLE_MAX,
};

fn encode(r: &Records<'_>) -> Vec<u8> {
    let mut out = vec![0u8; records_len(r).expect("within caps")];
    assert_eq!(write_records(&mut out, r), Some(out.len()));
    out
}

#[test]
fn records_split_back_as_they_were_written() {
    let anchor = [7u8; ANCHOR_LEN];
    let r = Records { enrol: b"cert", table: b"table bytes", anchor: &anchor, list: b"list" };
    let bytes = encode(&r);
    assert_eq!(bytes.len(), 16 + 4 + 11 + 32 + 4);
    assert_eq!(split_records(&bytes), Some(r));
    assert!(r.complete());
}

#[test]
fn an_empty_record_is_one_the_machine_does_not_hold() {
    let r = Records { enrol: &[], table: b"t", anchor: &[], list: b"l" };
    let bytes = encode(&r);
    let back = split_records(&bytes).expect("splits");
    assert_eq!(back, r);
    assert!(!back.complete());
    let none = Records { enrol: &[], table: &[], anchor: &[], list: &[] };
    assert_eq!(split_records(&encode(&none)), Some(none));
}

#[test]
fn a_frame_that_is_not_whole_does_not_split() {
    let anchor = [1u8; ANCHOR_LEN];
    let bytes = encode(&Records { enrol: b"e", table: b"t", anchor: &anchor, list: b"l" });
    for cut in 0..bytes.len() {
        assert_eq!(split_records(&bytes[..cut]), None, "cut at {cut}");
    }
    let mut longer = bytes.clone();
    longer.push(0);
    assert_eq!(split_records(&longer), None, "bytes after the last record");
}

#[test]
fn a_length_past_the_frame_does_not_split() {
    let mut bytes = vec![0u8; 16];
    bytes[..4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(split_records(&bytes), None);
}

#[test]
fn every_cap_is_held_on_both_sides() {
    let big = vec![0u8; LIST_MAX + 1];
    let anchor = [0u8; ANCHOR_LEN];
    let cases = [
        Records { enrol: &big[..ENROL_MAX + 1], table: &[], anchor: &[], list: &[] },
        Records { enrol: &[], table: &big[..TABLE_MAX + 1], anchor: &[], list: &[] },
        Records { enrol: &[], table: &[], anchor: &anchor[..31], list: &[] },
        Records { enrol: &[], table: &[], anchor: &[], list: &big[..] },
    ];
    for r in cases {
        assert_eq!(records_len(&r), None);
        assert_eq!(write_records(&mut vec![0u8; 1 << 21], &r), None);
        let mut raw = Vec::new();
        for f in [r.enrol, r.table, r.anchor, r.list] {
            raw.extend_from_slice(&(f.len() as u32).to_le_bytes());
            raw.extend_from_slice(f);
        }
        assert_eq!(split_records(&raw), None, "a record over its cap is refused when read");
    }
    let at_caps = Records {
        enrol: &big[..ENROL_MAX],
        table: &big[..TABLE_MAX],
        anchor: &anchor,
        list: &big[..LIST_MAX],
    };
    let len = records_len(&at_caps).expect("at the caps");
    assert!(len + 12 < 1 << 20, "a whole answer stays under the kernel's 1 MiB message");
}

#[test]
fn a_short_buffer_is_refused_not_overrun() {
    let r = Records { enrol: b"abc", table: &[], anchor: &[], list: &[] };
    let need = records_len(&r).expect("len");
    assert_eq!(write_records(&mut vec![0u8; need - 1], &r), None);
}
