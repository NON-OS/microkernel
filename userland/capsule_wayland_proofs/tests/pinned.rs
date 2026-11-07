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

//! A Linux window shown from its client's buffer, pinned in place: nothing
//! of it read out of the guest, the compositor told to draw the pin over
//! the window's frame, the buffer kept until the compositor lets it go, and
//! the window back on its own buffers when a pin cannot be had, when the
//! client stops drawing, or when the buffer it shows is destroyed.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::linux::call::set_clock;
use nonos_capsule_wayland_proofs::linux::guest_reads;

const W: i32 = 320;
const H: i32 = 200;
/// wl_buffer.release.
const RELEASE: u16 = 0;

fn pins(allow: bool) {
    nonos_libc::world::with(|w| w.allow_pins = allow);
}

/// A framed toplevel shown twice whole from buffer 101, copied, with buffers 101,
/// 103 and 105 painted, each in a pool of its own.
fn window() -> Client {
    set_clock(1_000);
    desk::with(|d| (d.width, d.height) = (W as u32 + 256, H as u32 + 256));
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("pinned")]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    for n in 0..3 {
        c.painted(SHM, 100 + 2 * n, 101 + 2 * n, (W, H), 0xFF20_40A0 + n);
    }
    commit(&mut c, 101, (0, 0, W, H));
    // Shown once more, whole: the first frame is painted again as the
    // window takes focus, so the commits after are drawn as they come.
    commit(&mut c, 101, (0, 0, W, H));
    c
}

fn commit(c: &mut Client, buffer: u32, b: (i32, i32, i32, i32)) -> Vec<Ev> {
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
    c.events()
}

fn released(evs: &[Ev], buffer: u32) -> bool {
    evs.iter().any(|e| e.object == buffer && e.opcode == RELEASE)
}

fn contents() -> Vec<(u64, u64)> {
    let calls = desk::with(|d| d.calls.clone());
    calls
        .iter()
        .filter_map(|c| match c {
            Call::Content(on, handle, _, _, _) => Some((*on, *handle)),
            _ => None,
        })
        .collect()
}

#[test]
fn a_pinned_window_reads_nothing_and_names_its_pin_to_the_compositor() {
    pins(true);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    guest_reads::take();
    for i in 0..8u32 {
        commit(&mut c, if i % 2 == 0 { 101 } else { 103 }, (10, 10, 40, 20));
    }
    let r = guest_reads::take();
    assert_eq!((r.calls, r.bytes), (0, 0), "no pixel read out of the guest");
    let shown = contents();
    assert!(shown.len() >= 8, "each commit named its pin: {shown:?}");
    assert!(shown.iter().rev().take(8).all(|&(on, h)| on != 0 && h != 0));
    let pinned = nonos_libc::world::with(|w| w.pinned);
    assert_eq!(pinned, 2, "one pin a buffer, kept across its commits");
}

#[test]
fn a_shown_buffer_is_kept_until_the_compositor_lets_it_go() {
    pins(true);
    let mut c = window();
    let evs = commit(&mut c, 103, (0, 0, W, H));
    assert!(!released(&evs, 103), "the layer draws it");
    let evs = commit(&mut c, 105, (0, 0, W, H));
    assert!(!released(&evs, 105));
    // The compositor composed a frame with 105 in 103's place.
    let _ = c.s.tick();
    let evs = c.events();
    assert!(released(&evs, 103), "103 back once nothing reads it");
    assert!(!released(&evs, 105), "105 is drawn still");
}

#[test]
fn with_no_pin_the_window_is_copied_as_before() {
    pins(false);
    let mut c = window();
    guest_reads::take();
    let evs = commit(&mut c, 103, (10, 10, 40, 20));
    assert!(released(&evs, 103), "copied, it is given back at once");
    assert!(guest_reads::take().bytes > 0);
    assert!(contents().is_empty(), "no content named");
}

#[test]
fn a_pin_refused_takes_the_window_back_to_its_own_buffers() {
    pins(true);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    assert!(contents().last().is_some_and(|c| c.1 != 0));
    pins(false);
    guest_reads::take();
    let evs = commit(&mut c, 105, (10, 10, 40, 20));
    assert!(guest_reads::take().bytes >= (W * H * 4) as u64, "105 drawn whole");
    assert!(released(&evs, 105), "copied, given back at once");
    assert_eq!(contents().last().map(|c| c.1), Some(0), "the layer draws no content");
    let _ = c.s.tick();
    assert!(released(&c.events(), 103), "the pinned one back once let go");
}

#[test]
fn a_window_left_on_one_buffer_gives_it_back_drawn_into_its_own() {
    pins(true);
    let mut c = one_buffer_window();
    pinned_one_buffer(&mut c);
    guest_reads::take();
    set_clock(1_000 + 300);
    let _ = c.s.tick();
    assert!(guest_reads::take().bytes >= (W * H * 4) as u64, "103 drawn into the window");
    assert_eq!(contents().last().map(|c| c.1), Some(0));
    let _ = c.s.tick();
    assert!(released(&c.events(), 103), "its client may draw in it again");
}

#[test]
fn a_destroyed_buffer_the_layer_draws_is_drawn_into_the_window_and_unpinned() {
    pins(true);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    let before = nonos_libc::world::with(|w| w.unpinned);
    guest_reads::take();
    c.req(103, 0, &[]);
    c.events();
    assert!(guest_reads::take().bytes >= (W * H * 4) as u64, "drawn while it is there");
    assert_eq!(contents().last().map(|c| c.1), Some(0));
    assert_eq!(nonos_libc::world::with(|w| w.unpinned), before + 1, "its pin given up");
}

/// What the screen shows of the window after 103 is committed whole: its
/// own buffer, and for a pinned one the compositor's content over it,
/// 103's pixels (all one opaque colour) from the corner it was named at.
fn screen(pinned: bool) -> (u32, Vec<u32>) {
    pins(pinned);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    let (w, h, mut px) = nonos_capsule_wayland_proofs::host::dump::pixels(&c.s.guest).unwrap();
    let calls = desk::with(|d| d.calls.clone());
    let named = calls.iter().rev().find_map(|c| match c {
        Call::Content(_, handle, at, _, _) => Some((*handle, *at)),
        _ => None,
    });
    assert_eq!(named.is_some_and(|n| n.0 != 0), pinned, "content named exactly when pinned");
    if let Some((handle, (cx, cy))) = named.filter(|n| n.0 != 0) {
        assert!(pinned, "content named only when pinned");
        assert_ne!(handle, 0);
        for y in cy..(cy + H as u32).min(h) {
            for x in cx..(cx + W as u32).min(w) {
                px[(y * w + x) as usize] = 0xFF20_40A1;
            }
        }
    }
    (w, px)
}

#[test]
fn a_pinned_window_on_screen_is_the_copied_one_pixel_for_pixel() {
    let (w, copied) = screen(false);
    let (pw, pinned) = screen(true);
    assert_eq!(w, pw);
    if let Some(i) = (0..copied.len()).find(|&i| copied[i] != pinned[i]) {
        panic!(
            "({}, {}): copied {:#010x}, pinned {:#010x}",
            i as u32 % w,
            i as u32 / w,
            copied[i],
            pinned[i]
        );
    }
}

/// The window's own buffer after 103 is committed with only `b` said to
/// have changed, with pins allowed or not and the content refused or not.
fn own_after(pinned: bool, refuse: bool, b: (i32, i32, i32, i32)) -> (u32, Vec<u32>) {
    pins(pinned);
    desk::with(|d| d.refuse_content = refuse);
    let mut c = window();
    commit(&mut c, 103, b);
    desk::with(|d| d.refuse_content = false);
    let (w, _, px) = nonos_capsule_wayland_proofs::host::dump::pixels(&c.s.guest).unwrap();
    (w, px)
}

#[test]
fn a_content_the_compositor_refuses_leaves_the_window_drawn_whole_as_copied() {
    let b = (10, 10, 40, 20);
    let (w, copied) = own_after(false, false, b);
    let (pw, shown) = own_after(true, true, b);
    assert_eq!(w, pw);
    assert!(contents().iter().all(|c| c.1 == 0), "no content taken");
    // Copied, 103's box lies over 101's pixels; refused, 103 is drawn whole,
    // as a pin would have shown it. No client pixel is the frame's ground.
    let client = |p: u32| p == 0xFF20_40A0 || p == 0xFF20_40A1;
    assert_eq!(copied.len(), shown.len());
    let n = copied.iter().filter(|&&p| client(p)).count();
    assert!(n >= (W * H) as usize, "the client's whole buffer is on the window");
    if let Some(i) = (0..copied.len()).find(|&i| client(copied[i]) && !client(shown[i])) {
        panic!(
            "({}, {}) shows {:#010x}, not the client's pixels",
            i as u32 % w,
            i as u32 / w,
            shown[i]
        );
    }
}

/// The display driver the desk's compositor names for scanout.
const DRIVER: u32 = 0x77;

fn pin_readers() -> Vec<(u32, u32)> {
    nonos_libc::world::with(|w| w.pin_readers.clone())
}

#[test]
fn a_pin_is_shared_with_the_scanout_driver_the_compositor_names() {
    pins(true);
    desk::with(|d| d.scanout_peer = DRIVER);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    commit(&mut c, 105, (0, 0, W, H));
    let compositor = nonos_app_skeleton::discover::lookup_service(b"compositor").unwrap().pid;
    let asked = pin_readers();
    assert!(!asked.is_empty(), "pinned");
    assert!(asked.iter().all(|&r| r == (compositor, DRIVER)), "{asked:?}");
}

#[test]
fn with_no_scanout_driver_named_a_pin_is_the_compositor_s_alone() {
    pins(true);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    let asked = pin_readers();
    assert!(!asked.is_empty() && asked.iter().all(|&(_, scanout)| scanout == 0), "{asked:?}");
}

#[test]
fn a_scanout_driver_the_kernel_refuses_leaves_the_pin_to_the_compositor_and_reads_nothing() {
    pins(true);
    desk::with(|d| d.scanout_peer = DRIVER);
    nonos_libc::world::with(|w| w.refuse_scanout = true);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    guest_reads::take();
    commit(&mut c, 105, (10, 10, 40, 20));
    let r = guest_reads::take();
    assert_eq!((r.calls, r.bytes), (0, 0), "shown from its pin, not copied");
    assert!(contents().last().is_some_and(|c| c.1 != 0), "the pin is the layer's content");
    let asked = pin_readers();
    let tail: Vec<u32> = asked.iter().rev().take(2).map(|r| r.1).collect();
    assert_eq!(tail, [0, DRIVER], "asked with the driver, then without: {asked:?}");
}

fn changed() -> Vec<(u64, (u32, u32, u32, u32))> {
    let calls = desk::with(|d| d.calls.clone());
    calls
        .iter()
        .filter_map(|c| match c {
            Call::Content(_, handle, _, changed, _) => Some((*handle, *changed)),
            _ => None,
        })
        .collect()
}

#[test]
fn a_swapchain_s_next_buffer_tells_the_compositor_only_the_box_its_client_said_changed() {
    pins(true);
    let mut c = window();
    desk::with(|d| d.calls.clear());
    commit(&mut c, 103, (0, 0, W, H));
    for i in 0..6u32 {
        commit(&mut c, if i % 2 == 0 { 101 } else { 103 }, (10, 10, 40, 20));
    }
    commit(&mut c, 105, (0, 0, W, H));
    let told = changed();
    assert_eq!(told.len(), 8, "every commit named its pin: {told:?}");
    let whole = (0, 0, W as u32, H as u32);
    assert_eq!(told[0].1, whole, "the first pin drawn over the ground: all of it");
    let handles: std::collections::BTreeSet<u64> = told[1..7].iter().map(|t| t.0).collect();
    assert_eq!(handles.len(), 2, "two buffers taking turns: {told:?}");
    assert!(
        told[1..7].iter().all(|t| t.1 == (10, 10, 40, 20)),
        "the box said, not the window: {told:?}"
    );
    assert_eq!(told[7].1, whole, "a buffer said changed whole is drawn whole");
}

/// Two buffers taking turns, each commit after the window sat idle: under
/// TCG, or keys typed at a person's pace, every commit comes so.
fn turns_after_idle(c: &mut Client, buffers: &[u32]) -> Vec<(u64, (u32, u32, u32, u32))> {
    desk::with(|d| d.calls.clear());
    let mut t = 1_000;
    for (i, &b) in buffers.iter().enumerate() {
        t += 300 + 10 * i as u64;
        set_clock(t);
        let _ = c.s.tick();
        let _ = c.s.tick();
        let _ = c.events();
        commit(c, b, (10, 10, 40, 20));
    }
    changed()
}

#[test]
fn a_client_with_a_spare_buffer_keeps_the_one_shown_past_idle_and_is_told_only_its_box() {
    pins(true);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    commit(&mut c, 101, (0, 0, W, H));
    let told = turns_after_idle(&mut c, &[103, 101, 103, 101, 103, 101]);
    assert_eq!(told.len(), 6, "one content a commit, none taken back: {told:?}");
    assert!(told.iter().all(|t| t.0 != 0), "the layer never cleared: {told:?}");
    assert!(told.iter().all(|t| t.1 == (10, 10, 40, 20)), "the box said, not the window: {told:?}");
    guest_reads::take();
    let _ = c.s.tick();
    assert_eq!(guest_reads::take().bytes, 0, "nothing read back out of the guest");
}

/// What one commit drew: its contents and presents (handle 0 for a
/// present), the bytes it read of the guest, and whether the client had
/// its buffer back by the commit's answer.
type Drawn = (Vec<(u64, (u32, u32, u32, u32))>, u64, bool);

/// The window with 103 its client's one buffer: 101 and 105, of its shape,
/// destroyed, as foot destroys its buffers of the old size on a resize.
fn one_buffer_window() -> Client {
    let mut c = window();
    c.req(101, 0, &[]);
    c.req(105, 0, &[]);
    c.events();
    c
}

/// The window's one buffer 103 shown from its pin: committed whole in a
/// run of frames, as only a run is pinned, the layer drawing it.
fn pinned_one_buffer(c: &mut Client) {
    for _ in 0..3 {
        commit(c, 103, (0, 0, W, H));
    }
    assert!(contents().last().is_some_and(|k| k.1 != 0), "shown from its pin");
}

/// One buffer, as foot keeps after a resize: given back at each idle, it
/// is drawn in again and committed `gaps` ms apart.
fn one_buffer_at(c: &mut Client, gaps: &[u64]) -> Vec<Drawn> {
    let mut t = 1_000;
    let mut seen = Vec::new();
    for &gap in gaps {
        t += gap;
        set_clock(t);
        let _ = c.s.tick();
        let _ = c.s.tick();
        let _ = c.events();
        desk::with(|d| d.calls.clear());
        guest_reads::take();
        let evs = commit(c, 103, (10, 10, 40, 20));
        let read = guest_reads::take().bytes;
        let calls = desk::with(|d| d.calls.clone());
        let drawn = calls.into_iter().filter_map(|call| match call {
            Call::Content(_, handle, _, box_, _) => Some((handle, box_)),
            Call::Present(_, _, box_) => Some((0, box_)),
            _ => None,
        });
        seen.push((drawn.collect(), read, released(&evs, 103)));
    }
    seen
}

#[test]
fn a_client_given_its_one_buffer_back_at_idle_has_each_slow_commit_copied_by_its_box() {
    pins(true);
    let mut c = one_buffer_window();
    pinned_one_buffer(&mut c);
    let shown = one_buffer_at(&mut c, &[300, 300, 300, 300]);
    let said = 40 * 20 * 4;
    for (drawn, read, back) in &shown {
        assert!(!drawn.is_empty(), "each commit drawn: {shown:?}");
        assert!(drawn.iter().all(|d| d.0 == 0), "no content named, none cleared: {shown:?}");
        assert!(drawn.iter().all(|d| d.1 .2 < W as u32), "never the whole window: {shown:?}");
        assert!(*read > 0 && *read <= said, "only its box read: {shown:?}");
        assert!(back, "copied, it is the client's at once: {shown:?}");
    }
}

#[test]
fn keys_typed_200_ms_apart_after_an_idle_give_back_are_each_copied_by_their_box() {
    pins(true);
    let mut c = one_buffer_window();
    pinned_one_buffer(&mut c);
    let shown = one_buffer_at(&mut c, &[300, 200, 200, 200, 200, 200]);
    let said = 40 * 20 * 4;
    for (drawn, read, back) in &shown {
        assert!(!drawn.is_empty(), "each commit drawn: {shown:?}");
        assert!(drawn.iter().all(|d| d.0 == 0), "no content named, none cleared: {shown:?}");
        assert!(drawn.iter().all(|d| d.1 .2 < W as u32), "never the whole window: {shown:?}");
        assert!(*read > 0 && *read <= said, "only its box read: {shown:?}");
        assert!(back, "copied, it is the client's at once: {shown:?}");
    }
}

#[test]
fn a_run_of_commits_after_an_idle_give_back_is_pinned_again() {
    pins(true);
    let mut c = one_buffer_window();
    pinned_one_buffer(&mut c);
    let shown = one_buffer_at(&mut c, &[300, 300, 16, 16, 16]);
    let (drawn, read, back) = shown.last().cloned().expect("the run's commit drawn");
    assert!(drawn.iter().any(|d| d.0 != 0), "shown from its pin: {shown:?}");
    assert_eq!(read, 0, "nothing read: {shown:?}");
    assert!(!back, "the layer draws it: {shown:?}");
}

#[test]
fn a_client_taking_turns_at_a_person_s_pace_from_its_first_frame_is_told_only_its_box() {
    pins(true);
    let mut c = window();
    let told = turns_after_idle(&mut c, &[103, 101, 103, 101, 103, 101, 103, 101]);
    assert!(told.len() >= 8, "every commit named its pin: {told:?}");
    let last = &told[told.len() - 6..];
    assert!(last.iter().all(|t| t.0 != 0), "the layer never cleared once both were seen: {told:?}");
    assert!(last.iter().all(|t| t.1 == (10, 10, 40, 20)), "the box said, not the window: {told:?}");
}

#[test]
fn a_client_that_has_a_spare_but_waits_on_its_shown_buffer_has_it_back_after_give_back_ms() {
    pins(true);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    commit(&mut c, 101, (0, 0, W, H));
    commit(&mut c, 103, (10, 10, 40, 20));
    set_clock(1_000 + 600);
    let _ = c.s.tick();
    assert!(!released(&c.events(), 103), "kept past idle: 101 is free to draw in");
    set_clock(1_000 + 1_100);
    let _ = c.s.tick();
    let _ = c.s.tick();
    assert!(released(&c.events(), 103), "given back after GIVE_BACK_MS all the same");
}

/// Two buffers taking turns, each commit `gap` ms after the last, the
/// bridge ticking every 100 ms between: what each commit drew and read, and
/// what the ticks before it drew, as for one_buffer_at.
fn turns_at(c: &mut Client, buffers: &[u32], gap: u64) -> Vec<(Drawn, usize)> {
    let mut t = 1_000;
    let mut seen = Vec::new();
    for &b in buffers {
        desk::with(|d| d.calls.clear());
        for _ in 0..gap / 100 {
            t += 100;
            set_clock(t);
            let _ = c.s.tick();
        }
        let _ = c.events();
        let ticked = desk::with(|d| d.calls.len());
        desk::with(|d| d.calls.clear());
        guest_reads::take();
        let evs = commit(c, b, (10, 10, 40, 20));
        let read = guest_reads::take().bytes;
        let calls = desk::with(|d| d.calls.clone());
        let drawn = calls.into_iter().filter_map(|call| match call {
            Call::Content(_, handle, _, box_, _) => Some((handle, box_)),
            Call::Present(_, _, box_) => Some((0, box_)),
            _ => None,
        });
        seen.push(((drawn.collect(), read, released(&evs, b)), ticked));
    }
    seen
}

#[test]
fn a_client_with_a_spare_committing_slower_than_give_back_ms_has_each_commit_copied_by_its_box() {
    pins(true);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    commit(&mut c, 101, (0, 0, W, H));
    let shown = turns_at(&mut c, &[103, 101, 103, 101, 103, 101], 1_200);
    let said = 40 * 20 * 4;
    for ((drawn, read, back), _) in &shown {
        assert!(!drawn.is_empty(), "each commit drawn: {shown:?}");
        assert!(drawn.iter().all(|d| d.0 == 0), "no pin named over a new ground: {shown:?}");
        assert!(drawn.iter().all(|d| d.1 .2 < W as u32), "never the whole window: {shown:?}");
        assert!(*read > 0 && *read <= said, "only its box read: {shown:?}");
        assert!(back, "copied, it is the client's at once: {shown:?}");
    }
    assert!(
        shown[1..].iter().all(|s| s.1 == 0),
        "given back once, then nothing to give: {shown:?}"
    );
}

#[test]
fn a_client_with_a_spare_committing_between_idle_and_give_back_ms_stays_on_its_pin() {
    pins(true);
    let mut c = window();
    commit(&mut c, 103, (0, 0, W, H));
    commit(&mut c, 101, (0, 0, W, H));
    let shown = turns_at(&mut c, &[103, 101, 103, 101, 103, 101], 700);
    for ((drawn, read, back), ticked) in &shown {
        assert_eq!(drawn.len(), 1, "one content a commit: {shown:?}");
        assert!(drawn[0].0 != 0, "shown from its pin: {shown:?}");
        assert_eq!(drawn[0].1, (10, 10, 40, 20), "the box said, not the window: {shown:?}");
        assert_eq!(*read, 0, "nothing read: {shown:?}");
        assert!(!back, "the layer draws it: {shown:?}");
        assert_eq!(*ticked, 0, "kept between commits, never given back: {shown:?}");
    }
}

#[test]
fn a_fresh_window_s_slow_commits_are_each_copied_by_their_box_from_the_first() {
    pins(true);
    // Drawn by the copy so far, as a window's first frames are, and never
    // shown from a pin: keys 2 s apart, as typed under an emulator.
    let mut c = one_buffer_window();
    let shown = one_buffer_at(&mut c, &[2_000, 2_000, 2_000, 2_000]);
    let said = 40 * 20 * 4;
    for (drawn, read, back) in &shown {
        assert!(!drawn.is_empty(), "each commit drawn: {shown:?}");
        assert!(drawn.iter().all(|d| d.0 == 0), "no content named, none cleared: {shown:?}");
        assert!(drawn.iter().all(|d| d.1 .2 < W as u32), "never the whole window: {shown:?}");
        assert!(*read > 0 && *read <= said, "only its box read: {shown:?}");
        assert!(back, "copied, it is the client's at once: {shown:?}");
    }
}

#[test]
fn a_run_of_commits_on_a_fresh_window_is_pinned() {
    pins(true);
    let mut c = one_buffer_window();
    let shown = one_buffer_at(&mut c, &[2_000, 16, 16, 16]);
    let (drawn, read, back) = shown.last().cloned().expect("the run's commit drawn");
    assert!(drawn.iter().any(|d| d.0 != 0), "shown from its pin: {shown:?}");
    assert_eq!(read, 0, "nothing read: {shown:?}");
    assert!(!back, "the layer draws it: {shown:?}");
}

#[test]
fn a_commit_30_ms_after_another_is_copied_by_its_box_not_pinned() {
    pins(true);
    // foot draws a key in two commits 30 ms apart; the pair is no run.
    let mut c = one_buffer_window();
    pinned_one_buffer(&mut c);
    let shown = one_buffer_at(&mut c, &[2_000, 30, 2_000, 30, 2_000, 30]);
    let said = 40 * 20 * 4;
    for (drawn, read, back) in &shown {
        assert!(drawn.iter().all(|d| d.0 == 0), "no content named, none cleared: {shown:?}");
        assert!(drawn.iter().all(|d| d.1 .2 < W as u32), "never the whole window: {shown:?}");
        assert!(*read > 0 && *read <= said, "only its box read: {shown:?}");
        assert!(back, "copied, it is the client's at once: {shown:?}");
    }
}

