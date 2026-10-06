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

//! A key held on a USB keyboard repeats after the delay, at the rate, until
//! it comes up or another key goes down, and never past the bound.

use crate::usb_key_repeat::{KeyRepeat, DELAY_MS, LIMIT_MS, RATE_MS};

const A: u8 = 0x04;
const B: u8 = 0x05;
const BACKSPACE: u8 = 0x2a;
const CAPS: u8 = 0x39;

/// The repeats a 1 ms poll loop sees from `from` to `to`.
fn run(r: &mut KeyRepeat, from: u64, to: u64) -> Vec<(u64, u8)> {
    (from..to).filter_map(|t| r.due(t).map(|k| (t, k))).collect()
}

#[test]
fn a_held_key_repeats_after_the_delay_at_the_rate() {
    let mut r = KeyRepeat::new();
    r.press(BACKSPACE);
    let got = run(&mut r, 1000, 1000 + DELAY_MS + 3 * RATE_MS + 1);
    let at: Vec<u64> = got.iter().map(|&(t, _)| t).collect();
    assert_eq!(
        at,
        [
            1000 + DELAY_MS,
            1000 + DELAY_MS + RATE_MS,
            1000 + DELAY_MS + 2 * RATE_MS,
            1000 + DELAY_MS + 3 * RATE_MS
        ]
    );
    assert!(got.iter().all(|&(_, k)| k == BACKSPACE));
}

#[test]
fn a_release_ends_the_repeat_and_a_new_key_takes_it_over() {
    let mut r = KeyRepeat::new();
    r.press(A);
    assert_eq!(run(&mut r, 0, DELAY_MS + 1).len(), 1);
    r.press(B);
    // B starts its own delay; A never repeats again.
    let got = run(&mut r, DELAY_MS + 1, 2 * DELAY_MS + 2);
    assert_eq!(got, [(2 * DELAY_MS + 1, B)]);
    // Releasing A, no longer repeating, leaves B alone.
    r.release(A);
    assert!(!run(&mut r, 2 * DELAY_MS + 2, 2 * DELAY_MS + RATE_MS + 2).is_empty());
    r.release(B);
    assert!(run(&mut r, 3 * DELAY_MS, 5 * DELAY_MS).is_empty());
}

#[test]
fn the_locks_do_not_repeat_and_end_a_repeat() {
    let mut r = KeyRepeat::new();
    r.press(A);
    r.press(CAPS);
    assert!(run(&mut r, 0, 3 * DELAY_MS).is_empty());
}

#[test]
fn a_late_tick_does_not_burst() {
    let mut r = KeyRepeat::new();
    r.press(A);
    assert_eq!(r.due(0), None);
    // The loop stalls for a second: one repeat, then the rate again.
    assert_eq!(r.due(1000 + DELAY_MS), Some(A));
    assert_eq!(r.due(1001 + DELAY_MS), None);
    assert_eq!(r.due(1000 + DELAY_MS + RATE_MS), Some(A));
}

#[test]
fn a_key_whose_release_never_came_stops_repeating() {
    let mut r = KeyRepeat::new();
    r.press(A);
    assert_eq!(r.due(0), None);
    assert_eq!(r.due(LIMIT_MS - 1), Some(A));
    assert_eq!(r.due(LIMIT_MS), None);
    assert!(run(&mut r, LIMIT_MS, LIMIT_MS + 10 * RATE_MS).is_empty());
}

#[test]
fn volume_steps_repeat_and_mute_and_power_act_once() {
    use crate::usb_key_repeat::repeats;
    // Held, Volume Up and Down keep stepping, as the PS/2 keyboard's own
    // repeat does; Mute would flip the sound at the rate and Power would ask
    // for a shutdown thirty times a second.
    assert!(repeats(0x80), "Volume Up");
    assert!(repeats(0x81), "Volume Down");
    assert!(!repeats(0x7f), "Mute");
    assert!(!repeats(0x66), "Power");

    let mut r = KeyRepeat::new();
    r.press(0x7f);
    assert!(run(&mut r, 0, LIMIT_MS).is_empty(), "a held Mute presses once");
    let mut r = KeyRepeat::new();
    r.press(0x80);
    let seen = run(&mut r, 0, DELAY_MS + 1 + RATE_MS);
    assert_eq!(seen.iter().map(|&(_, k)| k).collect::<Vec<_>>(), [0x80, 0x80]);
}
