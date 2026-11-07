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

//! wp_presentation: the clock said as it is bound, and each feedback
//! answered once for the commit it was asked for, presented when that
//! commit reached the window and discarded when it never will, then its id
//! given back.

use nonos_capsule_wayland_proofs::client::*;

const PRESENTATION: u32 = 30;
const OUTPUT: u32 = 31;
const FEEDBACK: u32 = 40;
const OTHER: u32 = 41;

const SYNC_OUTPUT: u16 = 0;
const PRESENTED: u16 = 1;
const DISCARDED: u16 = 2;

/// A shown window, with wp_presentation and a wl_output bound.
fn shown() -> Client {
    let mut c = Client::new();
    let globals = [
        ("wl_compositor", 6, COMPOSITOR),
        ("xdg_wm_base", 6, WM_BASE),
        ("wl_shm", 1, SHM),
        ("wp_presentation", 1, PRESENTATION),
        ("wl_output", 4, OUTPUT),
    ];
    c.bind(REGISTRY, &globals);
    let clock = c.events().into_iter().find(|e| e.object == PRESENTATION).expect("clock_id");
    assert_eq!((clock.opcode, clock.word(0)), (0, 1), "CLOCK_MONOTONIC");
    c.req(COMPOSITOR, 0, &[A::U(SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(XDG_SURFACE), A::U(SURFACE)]);
    c.req(XDG_SURFACE, 1, &[A::U(TOPLEVEL)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (400, 300));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

/// The display's delete_id events.
fn deleted(evs: &[Ev]) -> Vec<u32> {
    evs.iter().filter(|e| e.object == 1 && e.opcode == 1).map(|e| e.word(0)).collect()
}

#[test]
fn a_frame_shown_on_the_window_is_presented_on_the_guests_clock() {
    let mut c = shown();
    c.req(PRESENTATION, 1, &[A::U(SURFACE), A::U(FEEDBACK)]);
    assert!(to(&c.events(), FEEDBACK).is_empty(), "nothing before the commit");
    let before = nonos_capsule_wayland_proofs::linux::call::family_ms();
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 2, &[A::I(0), A::I(0), A::I(400), A::I(300)]);
    c.req(SURFACE, 6, &[]);
    let after = nonos_capsule_wayland_proofs::linux::call::family_ms();
    let evs = c.events();
    let fb = to(&evs, FEEDBACK);
    assert_eq!(fb.iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![SYNC_OUTPUT, PRESENTED]);
    assert_eq!(fb[0].word(0), OUTPUT);
    let p = fb[1];
    let ms = ((u64::from(p.word(0)) << 32 | u64::from(p.word(1))) * 1000)
        + u64::from(p.word(2)) / 1_000_000;
    assert!(before <= ms && ms <= after, "{before} <= {ms} <= {after}");
    // No refresh, no sequence, no flags: nothing claims vsync here.
    assert_eq!(&[p.word(3), p.word(4), p.word(5), p.word(6)], &[0, 0, 0, 0]);
    assert_eq!(deleted(&evs), vec![FEEDBACK], "its id goes back");
    assert!(c.s.guest.objects.get(FEEDBACK).is_none());
}

#[test]
fn a_frame_on_a_surface_not_on_the_window_is_discarded() {
    let mut c = shown();
    c.req(COMPOSITOR, 0, &[A::U(50)]);
    c.req(PRESENTATION, 1, &[A::U(50), A::U(OTHER)]);
    c.req(50, 6, &[]);
    let evs = c.events();
    assert_eq!(to(&evs, OTHER).iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![DISCARDED]);
    assert_eq!(deleted(&evs), vec![OTHER]);
}

#[test]
fn a_surface_destroyed_before_its_commit_discards_its_feedback() {
    let mut c = shown();
    c.req(COMPOSITOR, 0, &[A::U(50)]);
    c.req(PRESENTATION, 1, &[A::U(50), A::U(OTHER)]);
    c.req(50, 0, &[]);
    let evs = c.events();
    assert_eq!(to(&evs, OTHER).iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![DISCARDED]);
    assert!(deleted(&evs).contains(&OTHER));
}

/// The compositor reports frames shown: presented when the first frame after
/// the commit is, at its time, refresh and sequence, and not before.
#[test]
fn a_frame_is_presented_when_the_compositor_shows_it() {
    let mut c = shown();
    nonos_app_skeleton::desk::with(|d| {
        (d.frame_times, d.shown, d.refresh_ns) = (true, 41, 16_666_666);
        d.shown_ns = 7_250_000_123;
    });
    c.req(PRESENTATION, 1, &[A::U(SURFACE), A::U(FEEDBACK)]);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 2, &[A::I(0), A::I(0), A::I(400), A::I(300)]);
    c.req(SURFACE, 6, &[]);
    assert!(to(&c.events(), FEEDBACK).is_empty(), "not shown yet");
    assert_eq!(c.s.guest.scene.presenting.len(), 1, "waiting for the next frame");
    c.s.tick();
    let evs = c.events();
    let fb = to(&evs, FEEDBACK);
    assert_eq!(fb.iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![SYNC_OUTPUT, PRESENTED]);
    let p = fb[1];
    assert_eq!((p.word(0), p.word(1), p.word(2)), (0, 7, 250_000_123), "the time it was shown");
    assert_eq!((p.word(3), p.word(4), p.word(5), p.word(6)), (16_666_666, 0, 42, 0));
    assert_eq!(deleted(&evs), vec![FEEDBACK]);
}

/// A frame the compositor never shows is discarded once waited for long.
#[test]
fn a_frame_never_shown_is_discarded() {
    let mut c = shown();
    nonos_app_skeleton::desk::with(|d| (d.frame_times, d.covered) = (true, true));
    nonos_capsule_wayland_proofs::linux::call::set_clock(1_000);
    c.req(PRESENTATION, 1, &[A::U(SURFACE), A::U(FEEDBACK)]);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    assert!(c.s.tick().is_some_and(|ms| (1..=8).contains(&ms)), "asked again in half a refresh");
    nonos_capsule_wayland_proofs::linux::call::set_clock(1_200);
    c.s.tick();
    assert!(to(&c.events(), FEEDBACK).is_empty(), "still waiting");
    nonos_capsule_wayland_proofs::linux::call::set_clock(1_600);
    c.s.tick();
    let evs = c.events();
    assert_eq!(to(&evs, FEEDBACK).iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![DISCARDED]);
}

/// A feedback is asked after first a millisecond, then as long again as it
/// has waited, up to half a refresh: the compositor shows a frame on the
/// turn after its present, so it is told within about that, not half a
/// refresh late.
#[test]
fn a_frame_waiting_is_asked_for_soon_then_each_half_refresh() {
    use nonos_capsule_wayland_proofs::linux::call::set_clock;
    let mut c = shown();
    nonos_app_skeleton::desk::with(|d| (d.frame_times, d.covered) = (true, true));
    set_clock(1_000);
    c.req(PRESENTATION, 1, &[A::U(SURFACE), A::U(FEEDBACK)]);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    let mut asked = Vec::new();
    for at in [1_000, 1_001, 1_002, 1_004, 1_008, 1_016, 1_040] {
        set_clock(at);
        asked.push(c.s.tick().expect("asked again"));
    }
    assert_eq!(asked, vec![1, 1, 2, 4, 8, 8, 8], "soon, then each half refresh");
}
