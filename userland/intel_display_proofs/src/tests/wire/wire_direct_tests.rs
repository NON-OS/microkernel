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

//! SCANOUT on the taken-over engine: a client's buffer on plane 1 with the
//! band on plane 2, the buffer before it given back only once plane 1 no
//! longer reads it, FLUSH back to the primary whole, and every refusal
//! leaving the screen as it was. The model's watch counts any plane that
//! reaches a frame given back, and the GGTT entries still naming one.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::body::caps::{BAND_PLANE, DIRECT_SCANOUT};
use nonos_gfx_wire::errno::{EINVAL, ENODEV, EPERM};
use nonos_gfx_wire::{Rect, Scanout};

use crate::display::direct::layout;
use crate::engine::band_plane::{carve, trim, BLOCKS};
use crate::engine::ddb::Run;
use crate::engine::pipe::Pipe;
use crate::engine::wm_write::plane_buf_cfg;
use crate::tests::wire_rig::{h0, h0m, rig, shadow_bytes, Rig};

const A: u64 = 0xA1;
const B: u64 = 0xB2;
const A_IOVA: u64 = 0x5000_0000;
const B_IOVA: u64 = 0x5800_0000;
const PAGES: u64 = 2025;
const BAND: Rect = Rect { x: 1836, y: 0, width: 84, height: 16 };

fn shadow(bytes: &[u8]) -> Shadow<'_> {
    Shadow { bytes, width: 1920, height: 1080, stride: 7680, cursor_offset: 0 }
}

fn req(handle: u64) -> Scanout {
    Scanout { handle, serial: 1, width: 1920, height: 1080, stride: 7680, format: 1, band: BAND }
}

/// The rig with two client buffers the kernel would lend, A red and B
/// green, and plane 1's display buffer run as firmware leaves it.
fn direct_rig() -> (Rig, std::rc::Rc<std::cell::RefCell<crate::model::Gen9>>) {
    let (mut d, model) = rig();
    {
        let mut m = model.borrow_mut();
        m.fill(A_IOVA, PAGES, 0xFFCC_0000);
        m.fill(B_IOVA, PAGES, 0xFF00_CC00);
        m.set(plane_buf_cfg(Pipe::A), Run { start: 0, end: 864 }.cfg());
    }
    let f = &mut h0m(&mut d).frames;
    f.lendable = vec![(A, A_IOVA, PAGES * 4096), (B, B_IOVA, PAGES * 4096)];
    (d, model)
}

fn clean(model: &std::cell::RefCell<crate::model::Gen9>) {
    let m = model.borrow();
    assert_eq!(m.watch.reached, 0, "a plane reached a buffer given back");
    assert!(m.confine.stale.is_empty(), "a GGTT entry still names a buffer given back");
    assert!(m.confine.strays.is_empty(), "a GGTT entry names memory the driver does not hold");
}

#[test]
fn caps_offer_direct_scanout_with_the_band_plane_on_universal_planes() {
    let (d, _) = rig();
    let flags = d.caps().flags;
    assert_eq!(flags & (DIRECT_SCANOUT | BAND_PLANE), DIRECT_SCANOUT | BAND_PLANE);
}

#[test]
fn the_client_buffer_is_on_plane_one_and_the_band_over_it() {
    let (mut d, model) = direct_rig();
    let px = shadow_bytes(0xFF04_080C);
    d.scanout(0, &req(A), &shadow(&px)).unwrap();
    let m = model.borrow();
    assert_eq!(m.pixel(Pipe::A, 500, 500), Some(0xFFCC_0000), "plane 1 reads the client");
    assert_eq!(m.shown(Pipe::A, 500, 500), Some(0xFFCC_0000));
    assert_eq!(m.shown(Pipe::A, 1836, 0), Some(0xFF04_080C), "the band, not the client");
    assert_eq!(m.shown(Pipe::A, 1919, 15), Some(0xFF04_080C));
    assert_eq!(m.shown(Pipe::A, 1835, 0), Some(0xFFCC_0000), "only the band's rectangle");
    drop(m);
    assert_eq!(h0(&d).frames.lent, [A]);
    clean(&model);
}

#[test]
fn the_buffer_before_is_given_back_only_after_the_flip_to_the_next() {
    let (mut d, model) = direct_rig();
    let px = shadow_bytes(0xFF04_080C);
    d.scanout(0, &req(A), &shadow(&px)).unwrap();
    d.scanout(0, &req(B), &shadow(&px)).unwrap();
    assert_eq!(model.borrow().pixel(Pipe::A, 7, 7), Some(0xFF00_CC00));
    assert_eq!(h0(&d).frames.given, [A]);
    assert_eq!(h0(&d).frames.lent, [B]);
    d.scanout(0, &req(A), &shadow(&px)).unwrap();
    assert_eq!(model.borrow().pixel(Pipe::A, 7, 7), Some(0xFFCC_0000));
    assert_eq!(h0(&d).frames.given, [A, B]);
    clean(&model);
}

#[test]
fn the_same_buffer_again_moves_only_the_band() {
    let (mut d, model) = direct_rig();
    d.scanout(0, &req(A), &shadow(&shadow_bytes(0xFF04_080C))).unwrap();
    d.scanout(0, &req(A), &shadow(&shadow_bytes(0xFF35_D6A0))).unwrap();
    assert_eq!(h0(&d).frames.lent, [A], "not lent twice");
    assert!(h0(&d).frames.given.is_empty());
    assert_eq!(model.borrow().shown(Pipe::A, 1900, 8), Some(0xFF35_D6A0), "the band's new marks");
    clean(&model);
}

#[test]
fn a_flush_brings_the_primary_back_whole_then_lets_the_client_go() {
    let (mut d, model) = direct_rig();
    let px = shadow_bytes(0xFF12_3456);
    d.scanout(0, &req(A), &shadow(&px)).unwrap();
    // The compositor composes whole on leaving; the driver copies whole
    // whatever it sends.
    let small = Rect { x: 0, y: 0, width: 8, height: 8 };
    d.flush(0, &[small], &shadow(&px)).unwrap();
    let m = model.borrow();
    assert_eq!(m.pixel(Pipe::A, 1000, 900), Some(0xFF12_3456), "the primary, everywhere");
    assert_eq!(m.shown(Pipe::A, 1900, 8), m.pixel(Pipe::A, 1900, 8), "the band plane is off");
    drop(m);
    assert!(h0(&d).frames.lent.is_empty());
    assert_eq!(h0(&d).frames.given, [A]);
    assert!(h0(&d).direct.shown.is_none());
    // And the next flush keeps both surfaces whole.
    d.flush(0, &[small], &shadow(&px)).unwrap();
    assert_eq!(model.borrow().pixel(Pipe::A, 1000, 900), Some(0xFF12_3456));
    clean(&model);
}

#[test]
fn a_refused_lend_leaves_the_screen_as_it_was() {
    let (mut d, model) = direct_rig();
    let px = shadow_bytes(0xFF04_080C);
    let before = model.borrow().pixel(Pipe::A, 500, 500);
    h0m(&mut d).frames.refuse = Some(ENODEV);
    assert_eq!(d.scanout(0, &req(A), &shadow(&px)), Err(ENODEV), "no IOMMU: never lent");
    h0m(&mut d).frames.refuse = None;
    assert_eq!(d.scanout(0, &req(0xC3), &shadow(&px)), Err(EPERM), "not shared with the driver");
    let wrong = Scanout { stride: 7676, ..req(A) };
    assert_eq!(d.scanout(0, &wrong, &shadow(&px)), Err(EINVAL));
    let small = Scanout { width: 1280, height: 720, stride: 5120, ..req(A) };
    assert_eq!(d.scanout(0, &small, &shadow(&px)), Err(EINVAL));
    assert_eq!(model.borrow().pixel(Pipe::A, 500, 500), before);
    assert!(h0(&d).frames.lent.is_empty() && h0(&d).direct.shown.is_none());
    clean(&model);
}

#[test]
fn the_band_and_two_slots_fit_between_the_cursor_and_the_surfaces() {
    let (band, slots) = layout((0, 1 << 20), 1 << 19, 2025).expect("room");
    assert_eq!(band, 64);
    assert_eq!(slots, [128, 128 + 2048]);
    assert!(slots.iter().all(|s| s % 64 == 0), "256 KiB aligned");
    assert_eq!(layout((0, 1 << 20), 128 + 4096 - 1, 2025), None, "a slot would meet a surface");
    assert_eq!(layout((1000, 3000), u32::MAX, 2025), None, "past the window");
}

#[test]
fn the_band_takes_its_run_from_the_end_of_plane_ones() {
    let run = Run { start: 0, end: 864 };
    let (one, two) = carve(run).expect("room");
    assert_eq!((one, two), (Run { start: 0, end: 864 - BLOCKS }, Run { start: 864 - BLOCKS, end: 864 }));
    assert_eq!(carve(Run { start: 0, end: BLOCKS * 5 - 1 }), None);
    let on = |blocks: u32| (1 << 31) | (2 << 14) | blocks;
    let levels = [on(20), on(60), on(855), on(100), 0, 0, 0, 0];
    assert_eq!(trim(levels, 856), [on(20), on(60), on(855), on(100), 0, 0, 0, 0]);
    assert_eq!(trim(levels, 855), [on(20), on(60), 0, 0, 0, 0, 0, 0], "and every level above");
}
