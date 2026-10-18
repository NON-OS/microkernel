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

//! K8b: the kernel stamps each event's source into the high byte of its
//! flags; no app sees it. Every delivery carries the modifier byte alone.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_POINTER_REL};

use crate::clients::world;
use crate::router_desk::*;

const A: u32 = 20;
const MOD_SHIFT: u16 = 1 << 0;
const KEY_ESC: u32 = 0x1B;

#[test]
fn a_key_reaches_its_window_without_the_source() {
    for source in [1u16, 7, 0x80, 0xFE, 0xFF] {
        let mut ctx = desk(800, 600);
        window(&mut ctx, A, (0, 0, 300, 300));
        world::set_focus(A);
        key_down(&mut ctx, u32::from(b'x'), MOD_SHIFT | (source << 8));
        let rx: Vec<Rx> = sent().into_iter().filter(|r| r.kind == INPUT_KIND_KEY_DOWN).collect();
        assert_eq!(rx.len(), 1, "source {source}");
        assert_eq!((rx[0].pid, rx[0].flags), (A, MOD_SHIFT), "source {source}");
    }
}

#[test]
fn a_pointer_event_reaches_its_window_without_the_source() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 800, 600));
    post(&mut ctx, INPUT_KIND_POINTER_REL, 0, 0x2A00, 5, 5);
    down(&mut ctx, LEFT);
    let rx = sent();
    assert!(rx.iter().any(|r| r.pid == A), "{rx:?}");
    assert!(rx.iter().all(|r| r.flags & 0xFF00 == 0), "{rx:?}");
}

#[test]
fn the_reserved_chord_still_holds_with_a_source() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    world::set_focus(A);
    key_down(&mut ctx, KEY_ESC, MOD_CTRL | MOD_ALT | (3 << 8));
    let rx: Vec<Rx> = sent().into_iter().filter(|r| r.kind == INPUT_KIND_KEY_DOWN).collect();
    assert_eq!(rx.len(), 1);
    assert_eq!((rx[0].pid, rx[0].flags), (SHELL, MOD_CTRL | MOD_ALT), "to the shell, unstamped");
}
