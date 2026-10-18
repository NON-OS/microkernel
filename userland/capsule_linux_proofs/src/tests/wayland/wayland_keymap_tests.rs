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

//! Every key the server sends is one the shipped keymap gives symbols to:
//! xkb numbers a key as its evdev code plus 8, names it in xkb_keycodes,
//! and gives it symbols in xkb_symbols. A key it does not define would
//! reach a client as nothing.

use crate::evdev::evdev_of;
use std::collections::{BTreeMap, BTreeSet};

const KEYMAP: &str = include_str!("../../../../linux_userland/keymap-us.xkb");

/// `<NAME> = code;` in xkb_keycodes, and `key <NAME>` in xkb_symbols.
fn named_and_given() -> (BTreeMap<u32, String>, BTreeSet<String>) {
    let (codes, symbols) = KEYMAP.split_once("xkb_symbols").expect("a symbols section");
    let names = codes.lines().filter_map(|l| {
        let (name, code) = l.trim().strip_suffix(';')?.split_once(" = ")?;
        let name = name.strip_prefix('<')?.strip_suffix('>')?;
        Some((code.trim().parse().ok()?, name.to_string()))
    });
    let given = symbols.lines().filter_map(|l| {
        // xkbcomp pads a short name: `key  <ESC>`.
        let rest = l.trim().strip_prefix("key")?.trim_start().strip_prefix('<')?;
        Some(rest.split('>').next()?.to_string())
    });
    (names.collect(), given.collect())
}

#[test]
fn every_key_sent_has_symbols_in_the_keymap() {
    let (names, given) = named_and_given();
    assert!(names.len() > 100 && given.len() > 100, "{} named, {} given", names.len(), given.len());
    let sent: BTreeSet<u32> = (0..0x80).chain(0x1000..0x1400).filter_map(evdev_of).collect();
    assert!(sent.len() > 80);
    for evdev in sent {
        let name = names.get(&(evdev + 8)).unwrap_or_else(|| panic!("evdev {evdev} unnamed"));
        assert!(given.contains(name), "evdev {evdev} <{name}> has no symbols");
    }
}

#[test]
fn the_keymap_is_us_with_its_modifiers() {
    assert!(KEYMAP.contains("xkb_keycodes \"evdev+aliases(qwerty)\""));
    assert!(KEYMAP.contains("xkb_symbols \"pc+us+inet(evdev)\""));
    for modifier in ["Shift", "Lock", "Control", "Mod1", "Mod2", "Mod4", "Mod5"] {
        assert!(KEYMAP.contains(modifier), "{modifier}");
    }
}
