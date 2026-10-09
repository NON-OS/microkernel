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

//! A hostile device, played by a seeded generator: configuration
//! descriptors and report streams, two hundred thousand rounds of each,
//! through the driver's own decode. Nothing may panic, and nothing may come
//! out that lies outside the defined key, button and position ranges.

use std::collections::BTreeSet;

use crate::config_blob::{config, summaries, HID, INTERRUPT};
use crate::descriptors::hid_bindings;
use crate::descriptors::types::HidKind;
use crate::hid::button_changes::button_changes;
use crate::hid::mouse_report::{mouse_event, MOUSE_BUTTONS};
use crate::hid::tablet_report::{tablet_report, TABLET_BUTTONS};
use crate::keyboard_tests::step;
use crate::protocol::MAX_HID_BINDINGS;
use crate::rng::Rng;

const ROUNDS: u32 = 200_000;

type Binding = (HidKind, u8, u8, u16);

/// A key as the HID keyboard page defines one, stated here apart from the
/// driver's own rule.
fn is_key(usage: u8) -> bool {
    matches!(usage, 0x04..=0xA4 | 0xB0..=0xDD)
}

/// The binding rules written out a second time, from the USB and HID
/// specifications rather than from the driver, so every fuzzed walk is
/// checked against an independent statement of what it should pick.
fn reference(raw: &[u8]) -> Option<Vec<Binding>> {
    if raw.len() < 9 || raw[0] < 9 || raw[1] != 0x02 {
        return None;
    }
    let total = usize::from(u16::from_le_bytes([raw[2], raw[3]]));
    if total < 9 || total > raw.len() {
        return None;
    }
    let walk = &raw[..total];
    let (mut at, mut iface, mut out) = (9usize, None, Vec::new());
    while walk.len() - at >= 2 {
        let len = usize::from(walk[at]);
        if len < 2 || len > walk.len() - at {
            return None;
        }
        let rec = &walk[at..at + len];
        match rec[1] {
            0x04 => iface = (len >= 9).then(|| [rec[2], rec[5], rec[6], rec[7]]),
            0x05 if len >= 7 => out.extend(iface.and_then(|i| endpoint(i, rec))),
            _ => {}
        }
        if out.len() == MAX_HID_BINDINGS {
            break;
        }
        at += len;
    }
    // A device with a boot keyboard or mouse keeps only those.
    if out.iter().any(|b| b.0 != HidKind::Tablet) {
        out.retain(|b| b.0 != HidKind::Tablet);
    }
    Some(out)
}

fn endpoint([number, class, subclass, protocol]: [u8; 4], rec: &[u8]) -> Option<Binding> {
    let (address, attributes) = (rec[2], rec[3]);
    let size = u16::from_le_bytes([rec[4], rec[5]]) & 0x07ff;
    let (kind, least) = match (class, subclass, protocol) {
        (0x03, 1, 1) => (HidKind::Keyboard, 8),
        (0x03, 1, 2) => (HidKind::Mouse, 3),
        (0x03, 1, _) => return None,
        (0x03, _, _) => (HidKind::Tablet, 5),
        _ => return None,
    };
    let interrupt_in = address & 0x80 != 0 && address & 0x0f != 0 && attributes & 0x03 == 0x03;
    (interrupt_in && (least..=1024).contains(&size)).then_some((kind, number, address, size))
}

/// A configuration a hostile device might send: plausible records mixed
/// with cut ones, records of any length and type, and a wTotalLength that
/// may or may not match.
fn hostile_config(rng: &mut Rng) -> Vec<u8> {
    let mut blob = config();
    for _ in 0..rng.below(12) {
        let r = rng.byte();
        blob = match rng.below(8) {
            0..=2 => {
                let class = rng.pick(&[HID, HID, HID, 0x08, r]);
                let subclass = rng.pick(&[0, 1, 1, 2, r]);
                let protocol = rng.pick(&[0, 1, 2, 3, r]);
                blob.iface(rng.byte(), class, subclass, protocol)
            }
            3..=5 => {
                let address = rng.pick(&[0x81, 0x82, 0x8f, 0x80, 0x01, r]);
                let attributes = rng.pick(&[INTERRUPT, INTERRUPT, 0x02, r]);
                let w = rng.word() as u16;
                let size = rng.pick(&[0, 2, 3, 4, 5, 7, 8, 9, 64, 1024, 1025, 0x7ff, 0x1808, w]);
                blob.ep(address, attributes, size, rng.byte())
            }
            6 => blob.hid(),
            _ => {
                let len = rng.pick(&[0, 1, 2, 3, 6, 7, 8, 9, r]);
                let mut rec = vec![len, rng.pick(&[0x04, 0x05, 0x21, r])];
                let body = rng.below(12);
                rec.extend(rng.bytes(body));
                blob.raw(&rec)
            }
        };
    }
    let size = blob.size() as u16;
    let total = match rng.below(8) {
        0 => rng.word() as u16,
        1 => size.saturating_sub(rng.below(16) as u16),
        2 => size.saturating_add(rng.below(16) as u16),
        _ => size,
    };
    let mut d = blob.build_with_total(total);
    if rng.one_in(16) {
        d.truncate(rng.below(d.len() + 1));
    }
    d
}

/// Any byte string, sometimes behind a header that lets the walk start.
fn hostile_bytes(rng: &mut Rng) -> Vec<u8> {
    let len = rng.below(600);
    let mut d = rng.bytes(len);
    if len >= 9 && rng.one_in(2) {
        d[0] = 9;
        d[1] = 0x02;
        let total = if rng.one_in(2) { len as u16 } else { rng.word() as u16 };
        d[2..4].copy_from_slice(&total.to_le_bytes());
    }
    d
}

fn check_config(raw: &[u8], round: u32) {
    let got = hid_bindings(raw);
    assert_eq!(got.as_deref().ok().map(summaries), reference(raw), "round {round}: {raw:02x?}");
    let Ok(bindings) = got else { return };
    assert!(bindings.len() <= MAX_HID_BINDINGS, "round {round}");
    for b in &bindings {
        let least = match b.kind {
            HidKind::Keyboard => 8,
            HidKind::Mouse => 3,
            HidKind::Tablet => 5,
        };
        assert!(b.endpoint_address & 0x80 != 0, "round {round}: not IN");
        assert!((1..=15).contains(&(b.endpoint_address & 0x0f)), "round {round}: endpoint 0");
        assert!((least..=1024).contains(&b.max_packet_size), "round {round}: {b:?}");
    }
}

#[test]
fn hostile_configuration_descriptors_bind_only_what_the_rules_allow() {
    let mut rng = Rng::seeded(0x5eed_0001);
    for round in 0..ROUNDS {
        check_config(&hostile_config(&mut rng), round);
    }
}

#[test]
fn hostile_byte_strings_bind_only_what_the_rules_allow() {
    let mut rng = Rng::seeded(0x5eed_0002);
    for round in 0..ROUNDS {
        check_config(&hostile_bytes(&mut rng), round);
    }
}

/// A keyboard report a hostile device might send: mostly whole, slots
/// drawn from a few keys so presses and releases repeat, with error codes,
/// reserved usages, empty slots and any byte mixed in.
fn hostile_key_report(rng: &mut Rng, keys: &[u8]) -> Vec<u8> {
    let len = if rng.one_in(10) { rng.below(17) } else { 8 };
    (0..len)
        .map(|slot| {
            if slot < 2 {
                return rng.byte();
            }
            match rng.below(10) {
                0..=3 => rng.pick(keys),
                4 => 0,
                5 if rng.one_in(4) => rng.pick(&[1, 2, 3]),
                5 => 0,
                6 => rng.pick(&[0xA5, 0xAF, 0xDE, 0xE0, 0xE7, 0xE8, 0xFF]),
                _ => rng.byte(),
            }
        })
        .collect()
}

#[test]
fn a_hostile_keyboard_stream_presses_and_releases_exactly_what_its_reports_imply() {
    let mut rng = Rng::seeded(0x5eed_0003);
    let (mut held, mut down) = ([0u8; 6], BTreeSet::new());
    let mut keys = [0u8; 8];
    for round in 0..ROUNDS {
        if round % 4096 == 0 {
            keys.iter_mut().for_each(|k| *k = rng.byte());
            keys[0] = 0x04;
        }
        let raw = hostile_key_report(&mut rng, &keys);
        let (events, next, mods) = step(held, &raw);
        // Every event names a key and agrees with the stream so far: a
        // press only of a key up, a release only of a key down.
        for &(key, pressed) in &events {
            assert!(is_key(key), "round {round}: event for usage {key:#x}");
            let flipped = if pressed { down.insert(key) } else { down.remove(&key) };
            assert!(flipped, "round {round}: {key:#x} pressed {pressed} twice in a row");
        }
        let whole = raw.len() == 8;
        let says_nothing = !whole || raw[2..].iter().any(|k| (1..=3).contains(k));
        if says_nothing {
            assert!(events.is_empty(), "round {round}: {raw:02x?} made {events:?}");
            assert_eq!(next, held, "round {round}");
        } else {
            let named: BTreeSet<u8> = raw[2..].iter().copied().filter(|&k| is_key(k)).collect();
            assert_eq!(down, named, "round {round}: {raw:02x?}");
        }
        assert_eq!(mods, whole.then(|| raw[0]), "round {round}");
        let still: BTreeSet<u8> = next.iter().copied().filter(|&k| is_key(k)).collect();
        assert_eq!(still, down, "round {round}");
        held = next;
    }
}

#[test]
fn a_hostile_mouse_stream_stays_inside_the_defined_motion_and_buttons() {
    let mut rng = Rng::seeded(0x5eed_0004);
    let mut buttons = 0u8;
    for round in 0..ROUNDS {
        let len = rng.below(9);
        let mut raw = rng.bytes(len);
        if len >= 3 && rng.one_in(3) {
            raw[1..].iter_mut().for_each(|b| *b = 0);
        }
        if len >= 1 && rng.one_in(3) {
            raw[0] = buttons | (rng.byte() & 0xe0);
        }
        let Some(ev) = mouse_event(&raw, buttons) else {
            let quiet =
                len >= 3 && raw[0] & 0x1f == buttons && raw[1..len.min(4)].iter().all(|&b| b == 0);
            assert!(len < 3 || quiet, "round {round}: {raw:02x?} made nothing");
            continue;
        };
        assert!(len >= 3, "round {round}");
        assert_eq!(ev.buttons & !MOUSE_BUTTONS, 0, "round {round}");
        assert_eq!((ev.dx, ev.dy), (i16::from(raw[1] as i8), i16::from(raw[2] as i8)));
        assert!((-128..=127).contains(&ev.dx) && (-128..=127).contains(&ev.dy));
        assert_eq!(ev.dz, if len > 3 { raw[3] as i8 } else { 0 }, "round {round}");
        assert_eq!(ev.flags & !0b111, 0, "round {round}");
        check_buttons(buttons, ev.buttons, MOUSE_BUTTONS, 5, round);
        buttons = ev.buttons;
    }
}

#[test]
fn a_hostile_tablet_stream_stays_inside_the_logical_range_and_buttons() {
    let mut rng = Rng::seeded(0x5eed_0005);
    let mut buttons = 0u8;
    for round in 0..ROUNDS {
        let len = rng.below(9);
        let raw = rng.bytes(len);
        let Some(t) = tablet_report(&raw) else {
            assert!(len < 5, "round {round}: {raw:02x?} made nothing");
            continue;
        };
        assert!((0..=0x7fff).contains(&t.x) && (0..=0x7fff).contains(&t.y), "round {round}");
        assert_eq!(t.buttons & !TABLET_BUTTONS, 0, "round {round}");
        assert!((-128..=127).contains(&t.wheel), "round {round}");
        check_buttons(buttons, t.buttons, TABLET_BUTTONS, 3, round);
        buttons = t.buttons;
    }
}

/// Every button event between two button bytes names a button from 1 to
/// `top`, at most once, in the direction it went.
fn check_buttons(previous: u8, current: u8, mask: u8, top: u32, round: u32) {
    let mut seen = 0u8;
    button_changes(previous, current, mask, |button, down| {
        assert!((1..=top).contains(&button), "round {round}: button {button}");
        let bit = 1u8 << (button - 1);
        assert_eq!(seen & bit, 0, "round {round}: button {button} twice");
        seen |= bit;
        assert_eq!(down, current & bit != 0, "round {round}: button {button}");
    });
    assert_eq!(seen, (previous ^ current) & mask, "round {round}");
}
