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
//! A restarted driver believes nothing a dead one left lit: a
//! surface reaching into the GGTT entries its windows take is no firmware
//! picture to keep clear of. It is mapped over and never shown again;
//! firmware's own, low in the GGTT, is still kept clear of.

use crate::engine::ggtt_pte::Pte;
use crate::engine::pipe::Pipe;
use crate::engine::plane_regs as r;
use crate::engine::scanout::{leftover, Scanout};
use crate::engine::takeover::read_output;
use crate::model::Polls;
use crate::tests::scanout_tests::{setup, TOTAL};

const SECRET: u32 = 0xFF5E_C437;

#[test]
fn a_dead_driver_s_lit_surface_is_mapped_over_and_never_shown() {
    let (mut m, a, b) = setup();
    let top = (TOTAL - 2025) / 64 * 64;
    m.fill(0x3000_0000, 2025, SECRET);
    let dead: Vec<u64> = (0..2025).map(|i| 0x3000_0000 + i * 4096).collect();
    crate::engine::ggtt::map(&mut m, Pte::Wide, top, &dead, TOTAL).unwrap();
    m.set(r::plane_surf(Pipe::A, 1), top * 4096);
    m.set(r::plane_surflive(Pipe::A, 1), top * 4096);
    let out = read_output(&mut m, &crate::engine::gen::GEN9, Pipe::A).unwrap();
    assert_eq!(m.pixel(Pipe::A, 3, 3), Some(SECRET), "the dead driver's picture");
    assert!(leftover(&out.plane, TOTAL / 2) && !leftover(&out.plane, u32::MAX));
    let s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], TOTAL / 2, &Polls::new(50)).unwrap();
    assert_eq!(s.surfaces[0], top * 4096, "mapped over, not kept clear of");
    assert_eq!(m.pixel(Pipe::A, 1919, 1079), Some(0xFF00_00AA));
    assert!((0..2025).all(|i| m.ggtt[(top + i) as usize] & !0xFFF == a[i as usize]));
    let (mut m, a, b) = setup();
    crate::engine::ggtt::map(&mut m, Pte::Wide, top, &dead, TOTAL).unwrap();
    m.set(r::plane_surflive(Pipe::A, 1), top * 4096);
    let out = read_output(&mut m, &crate::engine::gen::GEN9, Pipe::A).unwrap();
    let s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], u32::MAX, &Polls::new(50)).unwrap();
    assert!(s.surfaces.iter().all(|&g| g / 4096 + 2025 <= top), "not ours: kept clear of");
}

#[test]
fn firmware_s_picture_low_in_the_ggtt_is_still_kept_clear_of() {
    let (mut m, a, b) = setup();
    let out = read_output(&mut m, &crate::engine::gen::GEN9, Pipe::A).unwrap();
    assert!(!leftover(&out.plane, TOTAL / 2));
    let fw = (out.plane.stride * out.plane.height).div_ceil(4096);
    assert!(!leftover(&out.plane, fw) && leftover(&out.plane, fw - 1), "its last page decides");
    let s = Scanout::start(&mut m, &out, TOTAL, [&a, &b], TOTAL / 2, &Polls::new(50)).unwrap();
    assert!(s.surfaces.iter().all(|&g| g / 4096 >= fw));
    assert_eq!(m.ggtt[0] & !0xFFF, 0x9000_0000, "firmware's entries kept");
}
