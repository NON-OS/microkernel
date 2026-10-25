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

//! A level as a row says it, from a table built once, so the row's text
//! lives as long as the page that shows it.

/// "0%" to "100%", each written once so a row can hold it for good.
static PERCENT: [([u8; 4], u8); 101] = table();

const fn table() -> [([u8; 4], u8); 101] {
    let mut t = [([0u8; 4], 0u8); 101];
    let mut v = 0;
    while v <= 100 {
        let (d, mut s) = ([(v / 100) as u8, (v / 10 % 10) as u8, (v % 10) as u8], [0u8; 4]);
        let skip = if v >= 100 {
            0
        } else if v >= 10 {
            1
        } else {
            2
        };
        let mut i = skip;
        while i < 3 {
            s[i - skip] = b'0' + d[i];
            i += 1;
        }
        s[3 - skip] = b'%';
        t[v] = (s, (4 - skip) as u8);
        v += 1;
    }
    t
}

pub fn percent_of(level: u8) -> &'static str {
    let (text, n) = &PERCENT[level.min(100) as usize];
    core::str::from_utf8(&text[..*n as usize]).unwrap_or("?")
}
