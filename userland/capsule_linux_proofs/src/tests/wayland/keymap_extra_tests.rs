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

//! The keys added to the shipped keymap for the characters a US key does
//! not give: each named by the keymap's own keycodes, given its character
//! at one level, and on a code the server sends for nothing else.

use crate::evdev::evdev_of;
use crate::keymap_extra::{extend, FIRST};
use std::collections::BTreeSet;

const KEYMAP: &str = include_str!("../../../../linux_userland/keymap-us.xkb");

/// Every character the layouts give beyond US (nonos_keymap tables).
const LAYOUTS: &str = "¡£¦§¨ª¬°²´µ·º¿ÄÇÑÖÜßàäçèéìñòöùü€";

#[test]
fn every_character_the_layouts_give_has_a_key() {
    for c in LAYOUTS.chars() {
        let evdev = evdev_of(c as u32).unwrap_or_else(|| panic!("{c} has no key"));
        assert!(evdev >= FIRST, "{c} on {evdev}");
    }
    for c in (0xA0..=0xFFu32).chain([0x20AC]) {
        assert!(evdev_of(c).is_some(), "U+{c:04X}");
    }
}

#[test]
fn no_added_key_is_one_the_server_sends_for_anything_else() {
    let us: BTreeSet<u32> = (0..0x80).chain(0x1000..0x1400).filter_map(evdev_of).collect();
    let added: BTreeSet<u32> = (0xA0..=0xFFu32).chain([0x20AC]).filter_map(evdev_of).collect();
    assert_eq!(added.len(), 97, "one key each");
    assert!(us.is_disjoint(&added));
    assert!(added.iter().all(|&k| k + 8 <= 255), "within the keymap's keycodes");
}

#[test]
fn the_extended_keymap_overrides_each_added_key_by_its_name() {
    let out = String::from_utf8(extend(KEYMAP.as_bytes()).expect("extended")).expect("text");
    let (codes, symbols) = out.split_once("xkb_symbols").expect("a symbols section");
    for (i, c) in (0xA0..=0xFFu32).chain([0x20AC]).enumerate() {
        let code = FIRST + i as u32 + 8;
        let name = codes
            .lines()
            .find(|l| l.trim_end().ends_with(&format!("= {code};")))
            .and_then(|l| l.trim().strip_prefix('<')?.split('>').next())
            .unwrap_or_else(|| panic!("keycode {code} is named"));
        let line = format!(
            "override key <{name}> {{ type= \"ONE_LEVEL\", symbols[Group1]= [ U{c:04X} ] }};"
        );
        assert!(symbols.contains(&line), "{line}");
    }
    // Added inside the symbols section, which still ends the way it did.
    let section = &symbols[..symbols.find("\n};").expect("its end")];
    assert!(section.contains("override key"));
    assert!(out.starts_with(&KEYMAP[..KEYMAP.find("xkb_symbols").expect("symbols")]));
    assert!(out.ends_with(&KEYMAP[KEYMAP.rfind("xkb_geometry").unwrap_or(KEYMAP.len() - 3)..]));
}

#[test]
fn a_keymap_not_laid_out_as_xkbcomp_lays_one_out_is_left_alone() {
    assert_eq!(extend(b"xkb_keymap { };"), None);
    assert_eq!(extend(&[0xFF, 0xFE]), None);
}

/// The keymap extended as it was before the names were found in one pass:
/// a search of the keycodes for each added key.
fn extended_by_search(base: &str) -> Option<String> {
    let symbols = base.find("xkb_symbols")?;
    let end = symbols + base[symbols..].find("\n};")?;
    let mut keys = String::new();
    for (i, c) in (0xA0..=0xFFu32).chain([0x20AC]).enumerate() {
        let want = format!("= {};", FIRST + i as u32 + 8);
        let line = base[..symbols].lines().find(|l| l.trim_end().ends_with(want.as_str()));
        let Some(name) = line.and_then(|l| l.trim().strip_prefix('<')?.split('>').next()) else {
            continue;
        };
        keys.push_str(&format!(
            "    override key <{name}> {{ type= \"ONE_LEVEL\", symbols[Group1]= [ U{c:04X} ] }};\n"
        ));
    }
    (!keys.is_empty()).then(|| [&base[..end + 1], &keys, &base[end + 1..]].concat())
}

#[test]
fn one_pass_names_the_same_keys_as_a_search_for_each() {
    let out = extend(KEYMAP.as_bytes()).map(|b| String::from_utf8(b).expect("text"));
    assert_eq!(out, extended_by_search(KEYMAP));
}

#[test]
fn a_keycode_is_read_only_as_xkbcomp_writes_it() {
    // The first line ending `= 148;` names keycode 148, as the search did;
    // a leading zero, a sign or no space before the code is no such line.
    let cases = [
        "    <AB01> = 148;\n    <AB02> = 148;\n",
        "    <ZERO> = 0148;\n    <PLUS> = +148;\n    <TIGHT> =148;\n    <REAL> = 148;\n",
        "    indicator 1 = 148;\n    <LATE> = 148;\n",
        "    <SPACE>   = 148;   \n",
        "    <WIDE> = 1148;\n",
    ];
    for keycodes in cases {
        let base =
            format!("xkb_keycodes {{\n{keycodes}}};\nxkb_symbols {{\n    key <X> {{}};\n}};\n");
        let out = extend(base.as_bytes()).map(|b| String::from_utf8(b).expect("text"));
        assert_eq!(out, extended_by_search(&base), "{keycodes:?}");
    }
}

#[test]
fn the_keymap_is_read_and_extended_once_and_a_miss_is_not_kept() {
    let file = include_str!("../../../../capsule_linux/src/linux/wayland/keymap_file.rs");
    let at = file.find("pub fn load()").expect("load");
    let load = &file[at..at + file[at..].find("\n}\n").expect("end")];
    let kept = load.find("KEPT.load(Acquire)").expect("the kept one first");
    let read = load.find("read()?").expect("else the store");
    assert!(kept < read);
    // Kept only from null, once: a second keymap never replaces the first
    // while a keyboard may hold a reference to it.
    assert!(load.contains("KEPT.compare_exchange(null_mut(), raw, AcqRel, Acquire)"));
    assert!(!file.contains("KEPT.store("), "never set but by the exchange");
    assert!(!file.contains("RefCell"), "no borrow a second thread could break");
}
