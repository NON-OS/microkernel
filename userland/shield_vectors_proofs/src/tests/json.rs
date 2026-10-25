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

//! The pinned proof.json read back into what its format 7 form is made from,
//! the proof's bytes and its public words; and the serial lines a check
//! wrote, captured through the `&mut impl Write` the capsule's checks take.

use crate::serial::TAG;
use crate::vectors::Vector;

/// The proof bytes and the public words `v`'s pinned proof.json holds.
pub(super) fn parts(v: &Vector) -> (Vec<u8>, Vec<u64>) {
    let text = std::str::from_utf8(v.proof_json).expect("proof.json is UTF-8");
    let field = |key: &str, end: char| {
        let at = text.find(key).unwrap_or_else(|| panic!("{}: no {key}", v.name)) + key.len();
        &text[at..at + text[at..].find(end).expect("the field ends")]
    };
    let hex = field("\"proof\": \"0x", '"');
    let byte = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).expect("the proof is hex");
    let bytes = (0..hex.len()).step_by(2).map(byte).collect();
    let words = field("\"publics\": [", ']').split(", ");
    let publics = words.map(|w| w.parse().expect("a public word")).collect();
    (bytes, publics)
}

/// Whether `check` passed, and the lines it wrote.
pub(super) fn lines(check: impl FnOnce(&mut Vec<u8>) -> bool) -> (bool, String) {
    let mut out = Vec::new();
    let ok = check(&mut out);
    (ok, String::from_utf8(out).expect("the lines are UTF-8"))
}

/// The FAIL lines among `text`'s, each without the tag and the verdict.
pub(super) fn failed(text: &str) -> Vec<&str> {
    let fail = format!("{TAG} FAIL ");
    text.lines().filter_map(|l| l.strip_prefix(fail.as_str())).collect()
}

/// The PASS lines among `text`'s, each without the tag and the verdict.
pub(super) fn passed(text: &str) -> Vec<&str> {
    let pass = format!("{TAG} PASS ");
    text.lines().filter_map(|l| l.strip_prefix(pass.as_str())).collect()
}
