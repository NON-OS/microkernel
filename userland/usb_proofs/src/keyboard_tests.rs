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

//! Boot keyboard reports, as the interrupt endpoint hands them over: each
//! one must press exactly the keys it names that the report before it did
//! not, and release exactly the keys it no longer names.

use crate::hid::keyboard::boot_report::BootReport;
use crate::hid::keyboard::key_changes::key_changes;

/// One report through the driver's decode, from the keys `held`: the key
/// events it makes, the keys held after it, and its modifier byte.
pub fn step(held: [u8; 6], raw: &[u8]) -> (Vec<(u8, bool)>, [u8; 6], Option<u8>) {
    let Some(report) = BootReport::parse(raw) else {
        return (Vec::new(), held, None);
    };
    let mut events = Vec::new();
    let next = key_changes(&held, &report, |key, pressed| events.push((key, pressed)));
    (events, next, Some(report.modifiers))
}

/// A well-formed boot report holding `keys` with modifier byte `mods`.
pub fn report(mods: u8, keys: &[u8]) -> [u8; 8] {
    let mut r = [0u8; 8];
    r[0] = mods;
    r[2..2 + keys.len()].copy_from_slice(keys);
    r
}

const A: u8 = 0x04;
const B: u8 = 0x05;
const C: u8 = 0x06;

#[test]
fn a_key_is_pressed_by_the_report_naming_it_and_released_by_the_next_that_does_not() {
    let (ev, held, _) = step([0; 6], &report(0, &[A]));
    assert_eq!(ev, vec![(A, true)]);
    let (ev, held, _) = step(held, &report(0, &[]));
    assert_eq!(ev, vec![(A, false)]);
    assert_eq!(held, [0; 6]);
}

#[test]
fn a_key_named_by_report_after_report_is_pressed_once() {
    let (_, mut held, _) = step([0; 6], &report(0, &[A, B]));
    for _ in 0..100 {
        let (ev, next, _) = step(held, &report(0, &[A, B]));
        assert!(ev.is_empty(), "a held key made {ev:?}");
        held = next;
    }
    // The same keys in other slots are the same keys.
    let (ev, _, _) = step(held, &report(0, &[0, B, 0, A]));
    assert!(ev.is_empty());
}

#[test]
fn presses_come_in_slot_order_and_releases_in_held_order() {
    let (_, held, _) = step([0; 6], &report(0, &[A, B]));
    let (ev, held, _) = step(held, &report(0, &[C, B]));
    assert_eq!(ev, vec![(C, true), (A, false)]);
    let (ev, _, _) = step(held, &report(0, &[A]));
    assert_eq!(ev, vec![(A, true), (C, false), (B, false)]);
}

#[test]
fn six_keys_at_once_press_six_and_release_six() {
    let six = [0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    let (ev, held, _) = step([0; 6], &report(0, &six));
    assert_eq!(ev, six.iter().map(|&k| (k, true)).collect::<Vec<_>>());
    let (ev, _, _) = step(held, &report(0, &[]));
    assert_eq!(ev, six.iter().map(|&k| (k, false)).collect::<Vec<_>>());
}

#[test]
fn the_modifier_byte_rides_on_the_report_and_makes_no_key_event() {
    for mods in 0..=u8::MAX {
        let (ev, held, got) = step([0; 6], &report(mods, &[]));
        assert!(ev.is_empty());
        assert_eq!((held, got), ([0; 6], Some(mods)));
    }
}

#[test]
fn the_reserved_byte_is_never_read() {
    for reserved in 0..=u8::MAX {
        let mut r = report(0, &[A]);
        r[1] = reserved;
        assert_eq!(step([0; 6], &r).0, vec![(A, true)]);
    }
}

#[test]
fn every_key_usage_presses_and_releases_its_own_code() {
    for key in (0x04u8..=0xA4).chain(0xB0..=0xDD) {
        let (ev, held, _) = step([0; 6], &report(0, &[key]));
        assert_eq!(ev, vec![(key, true)], "usage {key:#x}");
        let (ev, _, _) = step(held, &report(0, &[]));
        assert_eq!(ev, vec![(key, false)], "usage {key:#x}");
    }
}

#[test]
fn a_usage_named_twice_in_one_report_is_pressed_once_and_released_once() {
    let (ev, held, _) = step([0; 6], &report(0, &[A, A, B, A]));
    assert_eq!(ev, vec![(A, true), (B, true)]);
    let (ev, _, _) = step(held, &report(0, &[B]));
    assert_eq!(ev, vec![(A, false)]);
    // All six slots naming one key.
    let (ev, held, _) = step([0; 6], &report(0, &[C; 6]));
    assert_eq!(ev, vec![(C, true)]);
    let (ev, _, _) = step(held, &report(0, &[]));
    assert_eq!(ev, vec![(C, false)]);
}

#[test]
fn a_rollover_report_keeps_the_keys_held_and_makes_no_event() {
    let (_, held, _) = step([0; 6], &report(0, &[A, B]));
    // ErrorRollOver, POSTFail and ErrorUndefined.
    for error in [0x01u8, 0x02, 0x03] {
        // Every slot, as the HID specification has a keyboard send it.
        let (ev, kept, mods) = step(held, &report(0x02, &[error; 6]));
        assert!(ev.is_empty(), "{error:#x} made {ev:?}");
        assert_eq!((kept, mods), (held, Some(0x02)), "{error:#x}");
        // The report after it is read against the keys held before it.
        let (ev, _, _) = step(kept, &report(0, &[A, C]));
        assert_eq!(ev, vec![(C, true), (B, false)], "after {error:#x}");
        // One error slot among usages says the same.
        let (ev, kept, _) = step(held, &report(0, &[A, error, C]));
        assert!(ev.is_empty(), "{error:#x} among keys made {ev:?}");
        assert_eq!(kept, held);
    }
}

#[test]
fn a_usage_the_keyboard_page_names_no_key_makes_no_event() {
    // Reserved usages, and the modifiers, which a boot report carries as
    // bits in its first byte rather than in a key slot.
    for key in (0xA5u8..=0xAF).chain(0xDE..=0xFF) {
        let (ev, held, _) = step([0; 6], &report(0, &[key, A]));
        assert_eq!(ev, vec![(A, true)], "usage {key:#x}");
        let (ev, _, _) = step(held, &report(0, &[]));
        assert_eq!(ev, vec![(A, false)], "usage {key:#x}");
    }
}

#[test]
fn every_byte_in_a_key_slot_is_a_key_only_when_the_keyboard_page_says_so() {
    for key in 0..=u8::MAX {
        let is_key = matches!(key, 0x04..=0xA4 | 0xB0..=0xDD);
        let (ev, held, _) = step([0; 6], &report(0, &[key]));
        let want = if is_key { vec![(key, true)] } else { vec![] };
        assert_eq!(ev, want, "usage {key:#x} pressed");
        // Whatever a slot held, a report naming nothing lifts only keys.
        let (ev, _, _) = step(held, &report(0, &[]));
        let want = if is_key { vec![(key, false)] } else { vec![] };
        assert_eq!(ev, want, "usage {key:#x} released");
    }
}

#[test]
fn a_report_that_is_not_eight_bytes_changes_nothing() {
    let (_, held, _) = step([0; 6], &report(0x01, &[A, B]));
    for len in (0..8).chain(9..=16) {
        let mut raw = report(0x04, &[C]).to_vec();
        raw.resize(len, 0);
        let (ev, kept, mods) = step(held, &raw);
        assert!(ev.is_empty(), "{len} bytes made {ev:?}");
        // Not even the modifier byte is taken from it.
        assert_eq!((kept, mods), (held, None), "{len} bytes");
    }
}

#[test]
fn an_empty_slot_is_no_key() {
    let (ev, held, _) = step([0; 6], &report(0, &[0, 0, 0, 0, 0, 0]));
    assert!(ev.is_empty());
    assert_eq!(held, [0; 6]);
}
