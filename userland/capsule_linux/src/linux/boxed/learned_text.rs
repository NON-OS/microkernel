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
//! A learn run's `nonos-box 1` file: `base strict` for a run that wrote
//! nothing and reached for nothing, otherwise `base folders` with a `write`
//! line for each folder and a `connect` line for each destination tried, or
//! `network none`. A folder or destination no file can name is said in a
//! comment instead, as is what the run was refused that no file allows.

use alloc::string::String;
use alloc::vec::Vec;

use super::dests::list;
use super::folders::folder;

/// The longest box word a policy file compiles to (agentproof_policy).
pub const WORD_MAX: usize = 255;

pub fn text(writes: &[Vec<u8>], tried: &[Vec<u8>], refused: &str) -> String {
    let named = |v: &[u8]| String::from_utf8_lossy(v).into_owned();
    let (ok_w, bad_w): (Vec<_>, Vec<_>) = writes.iter().partition(|w| folder(w).is_ok());
    let (ok_c, bad_c): (Vec<_>, Vec<_>) =
        tried.iter().partition(|d| !d.contains(&b',') && list(&[d]).is_ok());
    let mut s = String::from("nonos-box 1\n# learned from one run; check it before you use it\n");
    for v in bad_w.iter().chain(&bad_c) {
        s.push_str(&alloc::format!("# not named, as no file can name it: {}\n", printable(v)));
    }
    /* `allow:`, each item and its comma; `/tmp` when it names no folder. */
    let folders = ok_w.iter().map(|w| w.len() + 1).sum::<usize>().max(5);
    let word = 6 + folders + ok_c.iter().map(|d| d.len() + 5).sum::<usize>();
    if word > WORD_MAX + 1 {
        s.push_str("# its box word is over 255 bytes: name fewer destinations\n");
    }
    if !refused.is_empty() {
        s.push_str(&alloc::format!("# also refused, which no file allows: {refused}\n"));
    }
    if ok_w.is_empty() && ok_c.is_empty() {
        s.push_str("base strict\nnetwork none\n");
        return s;
    }
    s.push_str("base folders\n");
    match ok_w.is_empty() {
        true => s.push_str("# a file names at least one folder\nwrite /tmp\n"),
        false => ok_w.iter().for_each(|w| s.push_str(&alloc::format!("write {}\n", named(w)))),
    }
    match ok_c.is_empty() {
        true => s.push_str("network none\n"),
        false => ok_c.iter().for_each(|d| s.push_str(&alloc::format!("connect {}\n", named(d)))),
    }
    s
}

/// Shown as a terminal shows it: every byte that is not printable as `?`.
fn printable(v: &[u8]) -> String {
    v.iter().map(|b| if b.is_ascii_graphic() { *b as char } else { '?' }).collect()
}
