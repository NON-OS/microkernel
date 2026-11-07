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

//! The display's refresh as the compositor reports it (OP_FRAME_TIME): told
//! in wl_output's mode, and the period frame callbacks are paced at; 60 a
//! second when it reports none.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::linux::call::set_clock;

const OUTPUT: u32 = 30;
const MODE: u16 = 1;
const CALLBACK: u32 = 40;
const CALLBACK2: u32 = 41;
const REGISTRY2: u32 = 42;
const HZ_144_NS: u64 = 1_000_000_000 / 144;

/// wl_output's mode refresh, in millihertz, with the compositor reporting
/// `refresh_ns`.
fn mode_refresh(refresh_ns: u64) -> u32 {
    desk::with(|d| d.refresh_ns = refresh_ns);
    let mut c = Client::new();
    c.bind(REGISTRY, &[("wl_output", 4, OUTPUT)]);
    let evs = c.events();
    let mode = to(&evs, OUTPUT).into_iter().find(|e| e.opcode == MODE).expect("a mode");
    mode.word(3)
}

#[test]
fn the_mode_says_the_refresh_the_compositor_reports() {
    assert_eq!(mode_refresh(HZ_144_NS), 144_000);
    assert_eq!(mode_refresh(0), 60_000, "none reported");
    assert_eq!(mode_refresh(1), 60_000, "none believable");
}

#[test]
fn a_client_drawing_at_once_waits_a_144th_of_a_second_not_a_60th() {
    desk::with(|d| d.refresh_ns = HZ_144_NS);
    let mut c = toplevel(6);
    // A registry of its own: toplevel() holds REGISTRY live.
    c.bind(REGISTRY2, &[("wl_output", 4, OUTPUT)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (64, 64));
    set_clock(10_000);
    // Answered at once, idle before; the next is held for a refresh.
    for callback in [CALLBACK, CALLBACK2] {
        c.req(SURFACE, 3, &[A::U(callback)]);
        c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
        c.req(SURFACE, 6, &[]);
        c.events();
    }
    assert!(c.s.guest.objects.get(CALLBACK).is_none(), "the first answered");
    assert!(c.s.guest.objects.get(CALLBACK2).is_some(), "the second held");
    let wait = c.s.tick().expect("one held");
    assert!((1..=7).contains(&wait), "due in {wait} ms");
}
