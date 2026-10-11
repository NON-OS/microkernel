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

//! One line of userland/capsule_linux/policies.txt: a vendor program's store
//! path, its services, and optionally its calls. The build script and this
//! crate's tests read it alike, so what the build accepts is what they check.

use crate::mask::{
    EXEC, FILES_HOME, FILES_USER, JIT, MODELS_RO, NET_ANON, NET_LISTEN_LO, TTY, WAYLAND,
};
use crate::syscall_set::{SyscallSet, SET_LEN};

pub struct Line<'a> {
    pub store_path: &'a str,
    pub mask: u64,
    pub calls: SyscallSet,
}

const NAMES: [(&str, u64); 9] = [
    ("WAYLAND", WAYLAND),
    ("NET_ANON", NET_ANON),
    ("NET_LISTEN_LO", NET_LISTEN_LO),
    ("FILES_HOME", FILES_HOME),
    ("FILES_USER", FILES_USER),
    ("MODELS_RO", MODELS_RO),
    ("EXEC", EXEC),
    ("JIT", JIT),
    ("TTY", TTY),
];

/// None for a blank line or a comment; the line's fault otherwise.
pub fn parse(line: &str) -> Result<Option<Line<'_>>, String> {
    let mut words = line.split_whitespace();
    let Some(store_path) = words.next().filter(|w| !w.starts_with('#')) else {
        return Ok(None);
    };
    if !store_path.starts_with("/bin/") || store_path.len() == 5 || store_path[5..].contains('/') {
        return Err(format!("{store_path}: not a program in /bin"));
    }
    let (mut mask, mut calls) = (0, SyscallSet::ALL);
    for word in words {
        if let Some(hex) = word.strip_prefix("calls=") {
            calls = set_of(hex).ok_or_else(|| format!("{store_path}: calls= is not a set"))?;
            continue;
        }
        let bit = NAMES.iter().find(|(n, _)| *n == word).map(|(_, b)| *b);
        mask |= bit.ok_or_else(|| format!("{store_path}: no service is named {word}"))?;
    }
    Ok(Some(Line { store_path, mask, calls }))
}

/// 130 hex digits, byte 0 first, as `[PROVE] syscall buckets ok=` prints the set.
pub fn set_of(hex: &str) -> Option<SyscallSet> {
    if hex.len() != SET_LEN * 2 || !hex.is_ascii() {
        return None;
    }
    let mut b = [0u8; SET_LEN];
    for (i, out) in b.iter_mut().enumerate() {
        *out = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).ok()?;
    }
    SyscallSet::from_bytes(b).ok()
}
