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

//! Established timings I, II (Table 3.18) and III (Table 3.30).

use crate::build::{base, display, put, seal};
use crate::edid::Edid;
use crate::source::Source;
use crate::timing::Timing;

fn all(e: &Edid) -> Vec<Timing> {
    let mut v = Vec::new();
    e.for_each_timing(&mut |t| v.push(t));
    v
}

fn picture(t: &[Timing]) -> Vec<(u32, u32, u32)> {
    t.iter().map(|t| (t.width, t.height, t.refresh_mhz / 1000)).collect()
}

#[test]
fn each_bit_of_bytes_35_to_37_names_its_mode() {
    let mut b = base();
    b[35] = 0x20; // 640x480 at 60
    b[36] = 0x08 | 0x10; // 1024x768 at 60, and at 87 interlaced
    b[37] = 0x80 | 0x7F; // 1152x870 at 75, plus the maker's own bits
    seal(&mut b);
    let t = all(&Edid::parse(&b).unwrap());
    assert_eq!(picture(&t), [(640, 480, 60), (1024, 768, 87), (1024, 768, 60), (1152, 870, 75)]);
    assert!(t[1].interlaced && !t[2].interlaced);
    assert!(t.iter().all(|t| t.source == Source::Established && t.detail.is_none()));
}

#[test]
fn the_first_and_last_bits_of_table_3_18() {
    let mut b = base();
    b[35] = 0x80;
    b[36] = 0x01;
    seal(&mut b);
    let t = all(&Edid::parse(&b).unwrap());
    assert_eq!(picture(&t), [(720, 400, 70), (1280, 1024, 75)]);
}

#[test]
fn established_iii_reads_the_bitmap_after_its_revision_byte() {
    let mut body = [0u8; 13];
    body[0] = 0x0A; // byte 5: revision
    body[1] = 0x80; // byte 6 bit 7: 640x350 at 85
    body[2] = 0x80; // byte 7 bit 7: 1280x768 at 60, reduced blanking
    body[6] = 0x10; // byte 11 bit 4: 1920x1440 at 75, the last
    body[6] |= 0x0F; // reserved bits name nothing
    let mut d = display(0xF7, &[]);
    d[5..18].copy_from_slice(&body);
    let mut b = base();
    put(&mut b, 2, &d);
    let t = all(&Edid::parse(&b).unwrap());
    assert_eq!(picture(&t), [(640, 350, 85), (1280, 768, 60), (1920, 1440, 75)]);
    assert!(!t[0].reduced_blanking && t[1].reduced_blanking);
}
