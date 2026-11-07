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

//! The pin rule's shape, pages, grants and room.

use super::pin_rule::{
    in_grant, may_read, pin_frames, pin_pages, room, GuestPage, PinRefusal, PinShape, PAGE,
    PINS_PER_SUPERVISOR, READER,
};

const USER_END: u64 = 0x0000_8000_0000_0000;
const MAX_PAGES: usize = 16384;

fn shape(addr: u64, byte_len: u64, width: u32, height: u32, stride: u32) -> PinShape {
    PinShape { addr, byte_len, width, height, stride }
}

fn mapped(frame: u64) -> Option<GuestPage> {
    Some(GuestPage { frame, user: true, exec: false })
}

#[test]
fn a_window_of_whole_rows_from_a_page_boundary_is_its_pages() {
    let s = shape(0x4000_0000, 1280 * 4 * 720, 1280, 720, 1280 * 4);
    assert_eq!(pin_pages(s, USER_END, MAX_PAGES), Ok(900));
    let odd = shape(0x4000_0000, 100 * 4 * 3 + 7, 100, 3, 400);
    assert_eq!(pin_pages(odd, USER_END, MAX_PAGES), Ok(1));
}

#[test]
fn a_pin_off_a_page_boundary_is_refused() {
    for addr in [1, 0x4000_0004, 0x4000_0800, 0x4000_0FFF] {
        assert_eq!(
            pin_pages(shape(addr, 4096, 16, 16, 64), USER_END, MAX_PAGES),
            Err(PinRefusal::Shape)
        );
    }
}

#[test]
fn rows_that_do_not_fit_their_bytes_or_their_stride_are_refused() {
    let at = 0x1000;
    let bad = [
        shape(at, 0, 1, 1, 4),
        shape(at, 4096, 0, 1, 4),
        shape(at, 4096, 1, 0, 4),
        shape(at, 4096, 16, 1, 63),
        shape(at, 64 * 64 - 1, 16, 64, 64),
        shape(at, u64::MAX, u32::MAX, 1, u32::MAX),
        shape(at, u64::MAX, 1 << 30, 1, u32::MAX),
    ];
    for s in bad {
        assert_eq!(pin_pages(s, USER_END, MAX_PAGES), Err(PinRefusal::Shape), "{s:?}");
    }
}

#[test]
fn a_pin_past_the_user_half_or_wrapping_is_refused() {
    let last = USER_END - PAGE;
    assert_eq!(pin_pages(shape(last, 4096, 16, 64, 64), USER_END, MAX_PAGES), Ok(1));
    assert_eq!(
        pin_pages(shape(last, 4097, 16, 64, 64), USER_END, MAX_PAGES),
        Err(PinRefusal::Shape)
    );
    let top = u64::MAX & !(PAGE - 1);
    assert_eq!(
        pin_pages(shape(top, 8192, 16, 64, 64), u64::MAX, MAX_PAGES),
        Err(PinRefusal::Shape)
    );
}

#[test]
fn a_pin_of_more_pages_than_a_surface_holds_is_refused() {
    let len = MAX_PAGES as u64 * PAGE;
    assert_eq!(pin_pages(shape(0, len, 1024, 16384, 4096), USER_END, MAX_PAGES), Ok(MAX_PAGES));
    let over = shape(0, len + 1, 1024, 16384, 4096);
    assert_eq!(pin_pages(over, USER_END, MAX_PAGES), Err(PinRefusal::Shape));
}

#[test]
fn the_frames_come_back_in_page_order() {
    let pages = [mapped(0x9000), mapped(0x3000), mapped(0x7000)];
    assert_eq!(pin_frames(&pages), Ok(vec![0x9000, 0x3000, 0x7000]));
}

#[test]
fn a_page_the_guest_has_not_mapped_or_may_not_touch_is_refused() {
    let none = [mapped(0x1000), None, mapped(0x3000)];
    assert_eq!(pin_frames(&none), Err(PinRefusal::Unmapped));
    let prot_none = [mapped(0x1000), Some(GuestPage { frame: 0x2000, user: false, exec: false })];
    assert_eq!(pin_frames(&prot_none), Err(PinRefusal::Unmapped));
}

#[test]
fn a_page_the_guest_may_run_is_refused() {
    let code = [mapped(0x1000), Some(GuestPage { frame: 0x2000, user: true, exec: true })];
    assert_eq!(pin_frames(&code), Err(PinRefusal::Executable));
}

#[test]
fn a_frame_inside_a_dma_grant_is_seen_and_one_beside_it_is_not() {
    let grants = [(0x10_0000u64, 0x4000u64)];
    assert!(in_grant(&[0x10_0000], grants));
    assert!(in_grant(&[0x5000, 0x10_3000], grants));
    assert!(!in_grant(&[0x10_4000], grants));
    assert!(!in_grant(&[0xF_F000], grants));
    assert!(in_grant(&[u64::MAX & !0xFFF], [(u64::MAX - 0x2000, u64::MAX)]));
    assert!(!in_grant(&[0x1000], core::iter::empty()));
}

#[test]
fn a_supervisor_s_guests_hold_at_most_sixteen_surfaces() {
    assert!((0..PINS_PER_SUPERVISOR).all(room));
    assert!(!room(PINS_PER_SUPERVISOR));
    assert!(!room(usize::MAX));
}

#[test]
fn the_share_a_pin_makes_is_read_only_and_ends_with_its_reader() {
    use super::rights_rule::{grant, revoke_pid, share_write, Grant, Right, SHARE_WRITE};
    assert_eq!(share_write(0), Some(false), "a pin shares with no flags");
    assert_eq!(share_write(SHARE_WRITE), Some(true));
    let mut rights = Vec::new();
    assert_eq!(grant(&mut rights, Right { handle: 7, pid: 100, write: false }), Grant::New);
    assert!(rights.iter().all(|r| !r.write));
    revoke_pid(&mut rights, 100);
    assert!(rights.is_empty(), "a reader's exit takes its right back");
}

const DISPLAYS: &[&str] = &["driver.virtio_gpu0", "driver.bootfb0"];

#[test]
fn a_pin_is_read_by_the_compositor_and_scanned_out_by_a_display_driver_alone() {
    assert!(may_read(&[READER], DISPLAYS, false));
    assert!(may_read(&["compositor.reply", READER], DISPLAYS, false));
    assert!(may_read(&["driver.virtio_gpu0"], DISPLAYS, true));
    assert!(!may_read(&["driver.virtio_gpu0"], DISPLAYS, false), "a driver is no reader");
    assert!(!may_read(&[READER], DISPLAYS, true), "the compositor scans nothing out");
    for other in [&["wm"][..], &["linux"], &["driver.e1000_0"], &[], &["compositorx"]] {
        assert!(!may_read(other, DISPLAYS, false), "{other:?}");
        assert!(!may_read(other, DISPLAYS, true), "{other:?}");
    }
}

#[test]
fn a_pin_is_answered_for_by_the_supervisor_and_any_other_surface_by_its_owner() {
    use super::answerable::answerable;
    assert_eq!(answerable(41, Some(30)), 30, "a guest's pin: the supervisor that pinned it");
    assert_eq!(answerable(40, None), 40, "every other surface: its owner");
}
