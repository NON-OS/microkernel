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

//! What one key event posts, with flags added by its caller: a wired post
//! (nothing added) is what the driver posted before the argument existed,
//! field for field; what is added lands in the flags above the modifier
//! bits and nowhere else; the modifier bits are never touched by it.

use nonos_keymap::HeldKeys;

use crate::hid::key_post::{key_posts, Post, MODIFIER_BITS};
use crate::key_post_seq::{events, flags_before, xorshift};

#[test]
fn a_wired_post_is_what_it_was() {
    for seed in 1..20_000 {
        let (mut now, mut then) = (HeldKeys::new(), HeldKeys::new());
        for (ev, code) in events(seed) {
            let got = key_posts(ev, &mut now, code, 0);
            let p = then.event(u32::from(ev.scancode), !ev.pressed, code);
            let flags = flags_before(ev.modifiers, ev.caps);
            let first = p.release_first.map(|code| Post { down: false, flags, code });
            let want = [first, p.code.map(|code| Post { down: ev.pressed, flags, code })];
            assert_eq!(got, want, "seed {seed}: {ev:?}");
        }
    }
}

#[test]
fn what_a_caller_adds_lands_above_the_modifiers_and_nowhere_else() {
    for seed in 1..20_000 {
        let extra = xorshift(&mut (seed | 1)) as u16;
        let (mut wired, mut added) = (HeldKeys::new(), HeldKeys::new());
        for (ev, code) in events(seed) {
            let (w, a) =
                (key_posts(ev, &mut wired, code, 0), key_posts(ev, &mut added, code, extra));
            for (w, a) in w.iter().zip(a.iter()) {
                let (Some(w), Some(a)) = (w, a) else {
                    assert_eq!(w.is_some(), a.is_some(), "seed {seed}: a post appeared or went");
                    continue;
                };
                assert_eq!((a.down, a.code), (w.down, w.code), "seed {seed}: kind or code moved");
                assert_eq!(
                    a.flags & MODIFIER_BITS,
                    w.flags & MODIFIER_BITS,
                    "seed {seed}: modifiers"
                );
                assert_eq!(
                    a.flags & !MODIFIER_BITS,
                    extra & !MODIFIER_BITS,
                    "seed {seed}: {extra:#06x}"
                );
            }
        }
    }
}
