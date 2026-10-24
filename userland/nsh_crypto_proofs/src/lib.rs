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

//! Proofs for nsh_crypto.

#![cfg(test)]

mod tests;

pub fn de(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

pub fn arr<const N: usize>(s: &str) -> [u8; N] {
    de(s).try_into().unwrap()
}

/// The value after `key` on the line of vectors/schedule.txt that starts
/// with `key`.
pub fn sched(key: &str) -> String {
    let text = include_str!("../vectors/schedule.txt");
    let line = text.lines().find(|l| l.starts_with(&format!("{key} "))).expect(key);
    line.rsplit(' ').next().unwrap().to_string()
}
