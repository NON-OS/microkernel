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

//! The `[LINUX-TIME] start` line, pure so the host proofs hold it. Each
//! stage is milliseconds since the capsule began, or `-` when the run never
//! reached it.

use alloc::format;
use alloc::string::String;

/// The stages' words, in the order of start_time's slots.
pub const WORDS: [&str; 11] = [
    "read", "grant", "prep", "family", "proved", "loaded", "running", "libs", "out", "connect",
    "byte",
];

/// `up`: the capsule's start on the uptime clock; `at`: each stage's
/// milliseconds plus one, 0 for not reached; `mapped`: proofs of files mapped
/// executable; `end`: milliseconds to this line; `how`: how the run ended.
pub fn line(up: u64, at: &[u64; 11], mapped: u64, end: u64, how: &str) -> String {
    let mut out = format!("[LINUX-TIME] start up {up} ms");
    for (word, at) in WORDS.iter().zip(at) {
        match at.checked_sub(1) {
            Some(ms) => out.push_str(&format!(" {word} {ms}")),
            None => out.push_str(&format!(" {word} -")),
        }
    }
    out.push_str(&format!(" end {end} mapped {mapped} {how}\n"));
    out
}
