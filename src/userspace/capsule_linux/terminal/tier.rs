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

//! The tiers a terminal may ask for, by the word it names each one with.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// Every word a terminal may send. The empty word is the first of them. The
/// names the personality knows the tiers by are these with `qwen-` before
/// them, and those it checks against the table its measurement covers.
const TIERS: [&str; 4] = ["small", "medium", "large", "xlarge"];

/// The tier `word` names, as an index into the allowlist, or `None` for any
/// word outside it. NUL bytes after the word are not part of it.
pub(super) fn parse(word: &[u8]) -> Option<u8> {
    let end = word.iter().rposition(|&b| b != 0).map_or(0, |i| i + 1);
    let word = &word[..end];
    if word.is_empty() {
        return Some(0);
    }
    let i = TIERS.iter().position(|t| t.as_bytes() == word)?;
    u8::try_from(i).ok()
}

/// The run request for tier `index`, as the personality reads it with
/// `mk_args`: `run`, the shipped program, and `cli`, which asks for the
/// terminal form of the program rather than its window. The question never
/// travels here; it goes to the program's stdin.
pub(super) fn argv(index: u8) -> Option<Vec<String>> {
    let tier = TIERS.get(usize::from(index))?;
    Some(vec![String::from("run"), alloc::format!("qwen-{tier}"), String::from("cli")])
}
