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

//! Reading pasted text as the keys that would type it.
//!
//! A paste is a number, or a sum the keypad can enter left to right:
//! `12.5`, `-3`, `1,234.56`, `7 × 6 =`, `100/8-2`. It becomes the presses
//! that type it, so the calculator does with it exactly what it does with
//! the keys. Anything else (a word, a date, `3,5` that reads as either
//! three and a half or thirty five, a number with more whole digits than
//! the display carries) is refused whole, since entering part of it would
//! leave a different number on the display than the one copied.

use alloc::vec::Vec;

use crate::calc::op::Op;

/// One press of a pasted number or sum.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PasteKey {
    Digit(u8),
    Point,
    Op(Op),
    /// The sign key, pressed after the digits of a negative number.
    Negate,
    Equals,
}

/// Whole digits the display carries (actions/digit.rs ignores the rest).
pub const INTEGER_DIGITS: usize = 16;
/// The longest paste read, in characters; no number or sum the display
/// could show is longer.
pub const PASTE_MAX: usize = 96;

fn op_of(ch: char) -> Option<Op> {
    match ch {
        '+' => Some(Op::Add),
        '-' | '\u{2212}' => Some(Op::Sub),
        '*' | 'x' | 'X' | '\u{d7}' => Some(Op::Mul),
        '/' | '\u{f7}' => Some(Op::Div),
        _ => None,
    }
}

fn minus(ch: char) -> bool {
    ch == '-' || ch == '\u{2212}'
}

struct Reader<'a> {
    chars: &'a [char],
    at: usize,
}

impl Reader<'_> {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.at).copied()
    }

    fn skip_space(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.at += 1;
        }
    }
}

/// A decimal number at the reader, pushed as its keys. A ',' or '_' groups
/// thousands only between a digit and exactly three more.
fn decimal(r: &mut Reader, keys: &mut Vec<PasteKey>) -> Option<()> {
    let negative = match r.peek() {
        Some(c) if minus(c) => {
            r.at += 1;
            true
        }
        Some('+') => {
            r.at += 1;
            false
        }
        _ => false,
    };
    let (mut whole, mut any) = (0usize, false);
    while let Some(c) = r.peek() {
        if let Some(d) = c.to_digit(10) {
            whole += 1;
            any = true;
            keys.push(PasteKey::Digit(d as u8));
            r.at += 1;
        } else if (c == ',' || c == '_') && any && grouped(r) {
            r.at += 1;
        } else {
            break;
        }
    }
    if whole > INTEGER_DIGITS {
        return None;
    }
    if r.peek() == Some('.') {
        r.at += 1;
        keys.push(PasteKey::Point);
        while let Some(d) = r.peek().and_then(|c| c.to_digit(10)) {
            any = true;
            keys.push(PasteKey::Digit(d as u8));
            r.at += 1;
        }
    }
    if !any {
        return None;
    }
    if negative {
        keys.push(PasteKey::Negate);
    }
    Some(())
}

/// Whether the separator at the reader is followed by exactly three digits.
fn grouped(r: &Reader) -> bool {
    let next = |i: usize| r.chars.get(r.at + i).copied();
    (1..=3).all(|i| next(i).is_some_and(|c| c.is_ascii_digit()))
        && !next(4).is_some_and(|c| c.is_ascii_digit())
}

/// An integer in `radix` for programmer mode: an optional sign, the prefix
/// that names this base (0x, 0o, 0b) if any, and its digits.
fn integer(r: &mut Reader, radix: u32, keys: &mut Vec<PasteKey>) -> Option<()> {
    let negative = r.peek().is_some_and(minus);
    if negative {
        r.at += 1;
    }
    let prefix = match radix {
        16 => Some('x'),
        8 => Some('o'),
        2 => Some('b'),
        _ => None,
    };
    if r.peek() == Some('0') {
        let p = r.chars.get(r.at + 1).map(|c| c.to_ascii_lowercase());
        if p.is_some() && p == prefix {
            r.at += 2;
        }
    }
    let mut any = false;
    while let Some(d) = r.peek().and_then(|c| c.to_digit(radix)) {
        any = true;
        keys.push(PasteKey::Digit(d as u8));
        r.at += 1;
    }
    if !any {
        return None;
    }
    if negative {
        keys.push(PasteKey::Negate);
    }
    Some(())
}

/// The presses that type `text`, or None when it is not a number or a sum.
/// `radix` is the programmer mode's base when that mode is up, where a paste
/// is one integer in that base.
pub fn keys_for(text: &str, radix: Option<u32>) -> Option<Vec<PasteKey>> {
    let chars: Vec<char> = text.trim().chars().take(PASTE_MAX + 1).collect();
    if chars.is_empty() || chars.len() > PASTE_MAX {
        return None;
    }
    let mut r = Reader { chars: &chars, at: 0 };
    let mut keys = Vec::new();
    if let Some(radix) = radix {
        integer(&mut r, radix, &mut keys)?;
        r.skip_space();
        return (r.at == chars.len()).then_some(keys);
    }
    decimal(&mut r, &mut keys)?;
    loop {
        r.skip_space();
        let Some(c) = r.peek() else { break };
        if c == '=' {
            r.at += 1;
            r.skip_space();
            keys.push(PasteKey::Equals);
            break;
        }
        let op = op_of(c)?;
        r.at += 1;
        keys.push(PasteKey::Op(op));
        r.skip_space();
        decimal(&mut r, &mut keys)?;
    }
    (r.at == chars.len()).then_some(keys)
}
