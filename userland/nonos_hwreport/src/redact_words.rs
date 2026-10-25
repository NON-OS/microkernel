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

//! The words that put a whole log line out of the report.

/// Whole words: the line names one, ignoring case, and it is dropped.
const WORDS: [&[u8]; 18] = [
    b"ssid",
    b"bssid",
    b"essid",
    b"serial",
    b"sn",
    b"s/n",
    b"uuid",
    b"guid",
    b"mac",
    b"hwaddr",
    b"ether",
    b"wwn",
    b"eui64",
    b"nguid",
    b"imei",
    b"iccid",
    b"hostname",
    b"username",
];
/// Starts of words: "password", "passphrase", "/home/...".
const PREFIXES: [&[u8]; 4] = [b"passw", b"passphrase", b"/home", b"psk"];

fn lower(b: u8) -> u8 {
    b.to_ascii_lowercase()
}

fn at(line: &[u8], i: usize, word: &[u8]) -> bool {
    line.len() - i >= word.len()
        && line[i..i + word.len()].iter().map(|b| lower(*b)).eq(word.iter().copied())
}

fn letter(b: Option<&u8>) -> bool {
    b.is_some_and(|b| b.is_ascii_alphanumeric())
}

/// Whether the line names a word that puts it out.
pub(crate) fn names_private(line: &[u8]) -> bool {
    (0..line.len()).any(|i| {
        let starts = i == 0 || !letter(line.get(i - 1));
        starts
            && (WORDS.iter().any(|w| at(line, i, w) && !letter(line.get(i + w.len())))
                || PREFIXES.iter().any(|p| at(line, i, p)))
    })
}
