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

//! A key and the reserved chord are their source's: modifiers count only
//! in the event that carries them, from the device that posted it, and a
//! press on one device is never released by another's key of the same code.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::router_desk::*;

const A: u32 = 20;
const B: u32 = 21;
const X: u32 = b'x' as u32;
const KEY_ESC: u32 = 0x1B;
const ONE: u16 = 1 << 8;
const TWO: u16 = 2 << 8;

fn keys(rx: &[Rx]) -> Vec<(u32, u16, u32)> {
    let key = |r: &&Rx| r.kind == INPUT_KIND_KEY_DOWN || r.kind == INPUT_KIND_KEY_UP;
    rx.iter().filter(key).map(|r| (r.pid, r.kind, r.code)).collect()
}

#[test]
fn two_sources_interleaving_a_chord_complete_none() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    world::set_focus(A);
    key_down(&mut ctx, KEY_ESC, MOD_CTRL | ONE);
    key_down(&mut ctx, KEY_ESC, MOD_ALT | TWO);
    let rx = keys(&sent());
    assert!(rx.iter().all(|&(pid, _, _)| pid == A), "half a chord each: {rx:?}");
    key_down(&mut ctx, KEY_ESC, MOD_CTRL | MOD_ALT | TWO);
    let last = *keys(&sent()).last().unwrap();
    assert_eq!(last, (SHELL, INPUT_KIND_KEY_DOWN, KEY_ESC), "one source, both modifiers");
}

#[test]
fn the_same_key_on_two_devices_is_two_keys() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    window(&mut ctx, B, (400, 0, 300, 300));
    world::set_focus(A);
    key_down(&mut ctx, X, ONE);
    world::set_focus(B);
    key_down(&mut ctx, X, TWO);
    key_up(&mut ctx, X, TWO);
    key_up(&mut ctx, X, ONE);
    assert_eq!(
        keys(&sent()),
        [
            (A, INPUT_KIND_KEY_DOWN, X),
            (B, INPUT_KIND_KEY_DOWN, X),
            (B, INPUT_KIND_KEY_UP, X),
            (A, INPUT_KIND_KEY_UP, X),
        ],
        "device two's press neither releases nor takes device one's"
    );
}
