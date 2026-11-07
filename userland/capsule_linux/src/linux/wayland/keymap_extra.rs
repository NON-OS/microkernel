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

//! Keys for the characters the US keymap has none for. The input driver
//! resolves a key through the person's layout (nonos_keymap: us, uk, de,
//! fr, es, it) and sends the character, and the server sends a client the
//! US key that gives it (evdev.rs). A character no US key gives (é, ü, ß,
//! ñ, £, €) had no key, and nothing was sent. Each now has a key of its own,
//! one level, on an evdev code the server sends for nothing else, added to
//! the shipped keymap as it is loaded: the printable half of Latin-1, which
//! holds every such character the layouts give, and the euro sign. Pure, so
//! the host proofs hold it.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// The first evdev code given to one: the codes from here are keys the
/// server never sends (evdev.rs, evdev_keys.rs), whose symbols are replaced.
pub const FIRST: u32 = 140;
/// The euro sign, after Latin-1's 96.
const EURO: u32 = 0x20AC;
const COUNT: u32 = 97;

/// The character's place among those given a key.
fn index(c: u32) -> Option<u32> {
    match c {
        0xA0..=0xFF => Some(c - 0xA0),
        EURO => Some(96),
        _ => None,
    }
}

/// The evdev code of the key for character `c`, when it has one.
pub fn evdev_of(c: u32) -> Option<u32> {
    index(c).map(|i| FIRST + i)
}

/// The character the `i`th key gives.
fn char_at(i: u32) -> u32 {
    if i == 96 {
        EURO
    } else {
        0xA0 + i
    }
}

/// `base`, the shipped keymap, with a key for each: an override in its
/// symbols section of the keycode xkb numbers its evdev code plus 8, named as
/// its keycodes section names it. None when the text is not laid out as an
/// xkbcomp keymap is, or names none of the codes.
pub fn extend(base: &[u8]) -> Option<Vec<u8>> {
    let text = core::str::from_utf8(base).ok()?;
    let symbols = text.find("xkb_symbols")?;
    let end = symbols + text[symbols..].find("\n};")?;
    let names = names(&text[..symbols]);
    let mut keys = String::new();
    for (i, name) in (0..COUNT).zip(names) {
        let Some(name) = name else { continue };
        let line = format!(
            "    override key <{name}> {{ type= \"ONE_LEVEL\", symbols[Group1]= [ U{:04X} ] }};\n",
            char_at(i)
        );
        keys.push_str(&line);
    }
    if keys.is_empty() {
        return None;
    }
    let mut out = Vec::with_capacity(base.len() + keys.len());
    out.extend_from_slice(&base[..end + 1]);
    out.extend_from_slice(keys.as_bytes());
    out.extend_from_slice(&base[end + 1..]);
    Some(out)
}

/// The name the keycodes section gives each added key's keycode, in one
/// pass over its lines: the first line that ends `= <code>;` names it, as
/// `<NAME> = <code>;`, or it has none when that line is not laid out so.
fn names(keycodes: &str) -> [Option<&str>; COUNT as usize] {
    let mut names = [None; COUNT as usize];
    let mut seen = [false; COUNT as usize];
    for line in keycodes.lines() {
        let Some(i) = keycode(line).and_then(|c| c.checked_sub(FIRST + 8)).filter(|&i| i < COUNT)
        else {
            continue;
        };
        let Some(seen) = seen.get_mut(i as usize).filter(|seen| !**seen) else { continue };
        *seen = true;
        names[i as usize] = line.trim().strip_prefix('<').and_then(|l| l.split('>').next());
    }
    names
}

/// The code a line ends `= <code>;` with, written as decimal digits with no
/// leading zero, as xkbcomp writes it.
fn keycode(line: &str) -> Option<u32> {
    let (_, digits) = line.trim_end().strip_suffix(';')?.rsplit_once("= ")?;
    let plain = digits.bytes().all(|b| b.is_ascii_digit()) && !digits.is_empty();
    let leading_zero = digits.len() > 1 && digits.starts_with('0');
    (plain && !leading_zero).then(|| digits.parse().ok()).flatten()
}
