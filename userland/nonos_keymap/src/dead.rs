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

//! Dead keys: the accent keys of the German, French and Spanish layouts,
//! which put an accent on the letter typed after them, as on any desktop.
//! The layout tables give such a key a dead code, past the last Unicode
//! scalar value so it is never taken for a character, and HeldKeys (held.rs)
//! turns it and the next key into one character before anything is posted:
//! no dead code leaves this crate.
//!
//! The accent and a letter it goes on give that letter (^ then e is ê); the
//! accent and Space, or the accent key twice, give the accent itself; any
//! other key drops the accent and is posted as it is. Every letter made here
//! is in Latin-1, which the Wayland bridge gives a key of its own
//! (capsule_linux wayland/keymap_extra.rs).

/// The first dead code: one past the last Unicode scalar value.
const FIRST: u32 = 0x0011_0000;

/// An accent a dead key puts on the next letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accent {
    Grave,
    Acute,
    Circumflex,
    Diaeresis,
}

const ACCENTS: [Accent; 4] = [Accent::Grave, Accent::Acute, Accent::Circumflex, Accent::Diaeresis];

/// The code a layout table gives the dead key for `accent`.
pub const fn dead(accent: Accent) -> u32 {
    FIRST + accent as u32
}

/// The accent `code` stands for, when it is a dead code.
pub fn accent_of(code: u32) -> Option<Accent> {
    let i = code.checked_sub(FIRST)?;
    ACCENTS.get(usize::try_from(i).ok()?).copied()
}

/// The accent on its own: what the dead key types before Space, or twice.
pub fn spacing(accent: Accent) -> u32 {
    match accent {
        Accent::Grave => b'`' as u32,
        Accent::Acute => 0x00B4,
        Accent::Circumflex => b'^' as u32,
        Accent::Diaeresis => 0x00A8,
    }
}

/// `accent` on the character `c`: the accented letter, the accent itself
/// for Space, None for anything it does not go on.
pub fn compose(accent: Accent, c: u32) -> Option<u32> {
    if c == b' ' as u32 {
        return Some(spacing(accent));
    }
    let c = u8::try_from(c).ok().filter(u8::is_ascii_alphabetic)?;
    let upper = c.is_ascii_uppercase();
    let lower = c.to_ascii_lowercase();
    let small = match (accent, lower) {
        (Accent::Grave, b'a') => 0xE0,
        (Accent::Grave, b'e') => 0xE8,
        (Accent::Grave, b'i') => 0xEC,
        (Accent::Grave, b'o') => 0xF2,
        (Accent::Grave, b'u') => 0xF9,
        (Accent::Acute, b'a') => 0xE1,
        (Accent::Acute, b'e') => 0xE9,
        (Accent::Acute, b'i') => 0xED,
        (Accent::Acute, b'o') => 0xF3,
        (Accent::Acute, b'u') => 0xFA,
        (Accent::Acute, b'y') => 0xFD,
        (Accent::Circumflex, b'a') => 0xE2,
        (Accent::Circumflex, b'e') => 0xEA,
        (Accent::Circumflex, b'i') => 0xEE,
        (Accent::Circumflex, b'o') => 0xF4,
        (Accent::Circumflex, b'u') => 0xFB,
        (Accent::Diaeresis, b'a') => 0xE4,
        (Accent::Diaeresis, b'e') => 0xEB,
        (Accent::Diaeresis, b'i') => 0xEF,
        (Accent::Diaeresis, b'o') => 0xF6,
        (Accent::Diaeresis, b'u') => 0xFC,
        // Capital Y with diaeresis is not in Latin-1: Y goes as it is.
        (Accent::Diaeresis, b'y') if !upper => 0xFF,
        _ => return None,
    };
    // Latin-1 puts each capital 0x20 below its small letter.
    Some(if upper { small - 0x20 } else { small })
}

/// Whether `code` is a modifier or lock key (the drivers' 0x1001 to 0x100B:
/// Ctrl, Shift, Alt, AltGr, Meta, Caps, Num and Scroll Lock), which a
/// pending accent waits through: Shift before a capital.
pub fn waits_through(code: u32) -> bool {
    (0x1001..=0x100B).contains(&code)
}
