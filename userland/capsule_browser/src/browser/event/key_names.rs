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

//! A key from the window as a page's keydown reads it: `key`, `code` and
//! the legacy `keyCode`.
//!
//! The keyboard drivers resolve the layout before the event leaves them,
//! so the window gets the character typed, not the key it is on. `key` is
//! that character, as the web defines it. `code` names a physical key, and
//! is read from the character as it sits on a US layout; on another layout
//! it is that guess, and empty for a character a US keyboard does not have.

use alloc::string::String;

use nonos_app_skeleton::{
    KEY_BACKSPACE, KEY_DELETE, KEY_DOWN, KEY_END, KEY_ENTER, KEY_ESC, KEY_F1, KEY_F12, KEY_HOME,
    KEY_INSERT, KEY_LEFT, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_RIGHT, KEY_TAB, KEY_UP,
};

/// A key's names as the web gives them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyNames {
    pub key: String,
    pub code: String,
    pub key_code: i32,
}

/// The names of the window's key `code`.
pub fn key_names(code: u32) -> KeyNames {
    let named = |key: &str, code: &str, n: i32| KeyNames {
        key: String::from(key),
        code: String::from(code),
        key_code: n,
    };
    match code {
        KEY_ENTER => named("Enter", "Enter", 13),
        KEY_TAB => named("Tab", "Tab", 9),
        KEY_BACKSPACE => named("Backspace", "Backspace", 8),
        KEY_ESC => named("Escape", "Escape", 27),
        KEY_UP => named("ArrowUp", "ArrowUp", 38),
        KEY_DOWN => named("ArrowDown", "ArrowDown", 40),
        KEY_LEFT => named("ArrowLeft", "ArrowLeft", 37),
        KEY_RIGHT => named("ArrowRight", "ArrowRight", 39),
        KEY_HOME => named("Home", "Home", 36),
        KEY_END => named("End", "End", 35),
        KEY_PAGE_UP => named("PageUp", "PageUp", 33),
        KEY_PAGE_DOWN => named("PageDown", "PageDown", 34),
        KEY_INSERT => named("Insert", "Insert", 45),
        KEY_DELETE => named("Delete", "Delete", 46),
        0x20 => named(" ", "Space", 32),
        f @ KEY_F1..=KEY_F12 => {
            let n = (f - KEY_F1 + 1) as i32;
            let name = alloc::format!("F{}", n);
            named(&name, &name, 111 + n)
        }
        c => match char::from_u32(c)
            .filter(|c| !c.is_control() && !(0x1000..=0x1FFF).contains(&(*c as u32)))
        {
            Some(ch) => {
                let (code, n) = us_key(ch);
                let mut key = String::new();
                key.push(ch);
                KeyNames { key, code, key_code: n }
            }
            None => named("Unidentified", "", 0),
        },
    }
}

/* The US key a character is typed on, and its keyCode. */
fn us_key(ch: char) -> (String, i32) {
    let up = ch.to_ascii_uppercase();
    if up.is_ascii_uppercase() {
        return (alloc::format!("Key{}", up), up as i32);
    }
    if ch.is_ascii_digit() {
        return (alloc::format!("Digit{}", ch), ch as i32);
    }
    let (code, n) = match ch {
        ')' => ("Digit0", 48),
        '!' => ("Digit1", 49),
        '@' => ("Digit2", 50),
        '#' => ("Digit3", 51),
        '$' => ("Digit4", 52),
        '%' => ("Digit5", 53),
        '^' => ("Digit6", 54),
        '&' => ("Digit7", 55),
        '*' => ("Digit8", 56),
        '(' => ("Digit9", 57),
        '-' | '_' => ("Minus", 189),
        '=' | '+' => ("Equal", 187),
        '[' | '{' => ("BracketLeft", 219),
        ']' | '}' => ("BracketRight", 221),
        '\\' | '|' => ("Backslash", 220),
        ';' | ':' => ("Semicolon", 186),
        '\'' | '"' => ("Quote", 222),
        ',' | '<' => ("Comma", 188),
        '.' | '>' => ("Period", 190),
        '/' | '?' => ("Slash", 191),
        '`' | '~' => ("Backquote", 192),
        _ => ("", 0),
    };
    (String::from(code), n)
}
