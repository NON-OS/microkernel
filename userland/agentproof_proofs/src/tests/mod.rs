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

mod cli_tests;
mod connect_tests;
mod corpus;
mod enforce_tests;
mod pin_tests;
mod random;
mod refusal_file_tests;
mod refusal_tests;
mod sweep_tests;

use agentproof_policy::boxed::hash::box_hash;
use agentproof_policy::boxed::preset::parse as word_parse;
use agentproof_policy::explain::hex;
use agentproof_policy::{compile, parse};

/// A file of these lines, a newline after each.
pub fn file(lines: &[&str]) -> Vec<u8> {
    lines.iter().flat_map(|l| l.bytes().chain([b'\n'])).collect()
}

pub fn hash_of(lines: &[&str]) -> String {
    hex(&compile(&parse(&file(lines)).unwrap()).unwrap().hash)
}

/// A `base folders` file with these folders.
pub fn folders(writes: &[&str]) -> String {
    let mut lines = vec!["nonos-box 1", "base folders", "network none"];
    let w: Vec<String> = writes.iter().map(|f| format!("write {f}")).collect();
    lines.extend(w.iter().map(String::as_str));
    hash_of(&lines)
}

pub fn word_hash(word: &[u8]) -> String {
    hex(&box_hash(&word_parse(word).unwrap()))
}
