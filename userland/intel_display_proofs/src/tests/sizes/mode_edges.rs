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

//! A monitor whose DisplayID lists a mode past the plane (8200 x 2000, a
//! type VII timing at 1 Hz, inside CDCLK): a mode set to it is refused
//! before a surface is sized or asked for, though the mode is the
//! monitor's own. (Its products would fit u32 anyway: CDCLK's 11-bit field
//! and the 1 Hz floor hold 4wh plus the stride's padding under 2^32.)

use crate::model::firmware::T1080;
use crate::tests::dp_rig::dp_rig;
use crate::tests::edid_blob::edid_1080p;
use crate::tests::wire_rig::h0m;
use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::EINVAL;

fn seal(b: &mut [u8]) {
    let n = b.len() - 1;
    b[n] = b[..n].iter().fold(0u8, |a, v| a.wrapping_add(*v)).wrapping_neg();
}

/// A DisplayID 2.0 extension with one type VII timing, `t` as active,
/// blank, front, sync for each direction, each stored less one.
fn displayid(clock_khz: u32, t: [u32; 8]) -> [u8; 128] {
    let mut rec = vec![0u8; 20];
    rec[..3].copy_from_slice(&(clock_khz - 1).to_le_bytes()[..3]);
    for (i, v) in t.iter().enumerate() {
        rec[4 + i * 2..6 + i * 2].copy_from_slice(&((v - 1) as u16).to_le_bytes());
    }
    let block = [&[0x22, 0, 20][..], &rec].concat();
    let mut b = [0u8; 128];
    b[0] = 0x70;
    let s = &mut b[1..1 + 5 + block.len()];
    (s[0], s[1]) = (0x20, block.len() as u8);
    s[4..4 + block.len()].copy_from_slice(&block);
    seal(s);
    seal(&mut b);
    b
}

#[test]
fn a_listed_mode_past_the_plane_is_refused_before_a_surface_is_asked_for() {
    let mut e = edid_1080p((600, 340));
    e[126] = 1;
    seal(&mut e[..128]);
    e.extend_from_slice(&displayid(16_804, [8200, 160, 48, 32, 2000, 10, 3, 5]));
    let (mut d, _, _) = dp_rig(T1080, e, 1 << 20);
    // No room for new surfaces: a set that got as far as asking is ENOMEM.
    h0m(&mut d).frames.next = None;
    let px = vec![0u8; 64];
    let s = Shadow { bytes: &px, width: 4, height: 4, stride: 16, cursor_offset: 0 };
    assert_eq!(d.set_mode(0, 8200, 2000, &s), Err(EINVAL));
}
