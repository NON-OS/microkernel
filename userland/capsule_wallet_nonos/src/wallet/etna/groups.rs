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

//! Long values broken into runs of eight so a person can check them and a
//! quiet alteration shows, and the two cuts the phones use. The same rules
//! as Groups.swift.

use alloc::string::String;
use alloc::vec::Vec;

pub const RUN: usize = 8;
const ENDS: usize = 6;
const MARK: &str = "\u{2026}";

pub fn grouped(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    let runs: Vec<String> = chars.chunks(RUN).map(|c| c.iter().collect()).collect();
    runs.join(" ")
}

/// A hash cut to its ends, as in 0x6def9b\u{2026}f3a4, so a forged prefix must
/// also match the suffix.
pub fn hash(value: &str) -> String {
    let n = value.chars().count();
    if !value.starts_with("0x") || n <= 12 {
        return String::from(value);
    }
    let head: String = value.chars().take(8).collect();
    let tail: String = value.chars().skip(n - 4).collect();
    alloc::format!("{head}{MARK}{tail}")
}

pub fn shortened(value: &str) -> String {
    let n = value.chars().count();
    if n <= ENDS * 2 + 1 {
        return String::from(value);
    }
    let head: String = value.chars().take(ENDS).collect();
    let tail: String = value.chars().skip(n - ENDS).collect();
    alloc::format!("{head}{MARK}{tail}")
}
