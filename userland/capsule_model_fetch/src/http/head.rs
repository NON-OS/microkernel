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

/* A response's status line and the headers the file is judged by. */

use alloc::string::String;

#[derive(Default)]
pub struct Head {
    pub status: u16,
    pub length: Option<u64>,
    /* Content-Range: the first byte and the whole length. */
    pub range: Option<(u64, u64)>,
    pub location: Option<String>,
    /* Any transfer coding other than none. */
    pub coded: bool,
}

pub fn parse(head: &[u8]) -> Option<Head> {
    let text = core::str::from_utf8(head).ok()?;
    let mut lines = text.split("\r\n");
    let status = lines.next()?.split(' ').nth(1)?.parse().ok()?;
    let mut out = Head { status, ..Head::default() };
    for line in lines {
        let Some((name, value)) = line.split_once(':') else { continue };
        let value = value.trim();
        match name.trim().to_ascii_lowercase().as_str() {
            "content-length" => out.length = value.parse().ok(),
            "content-range" => out.range = range(value),
            "location" => out.location = Some(String::from(value)),
            "transfer-encoding" => out.coded = !value.eq_ignore_ascii_case("identity"),
            _ => {}
        }
    }
    Some(out)
}

/* "bytes FIRST-LAST/TOTAL" as (FIRST, TOTAL). */
fn range(value: &str) -> Option<(u64, u64)> {
    let (span, total) = value.strip_prefix("bytes ")?.split_once('/')?;
    let (first, last) = span.split_once('-')?;
    let (first, last, total) =
        (first.parse().ok()?, last.parse::<u64>().ok()?, total.parse().ok()?);
    (last.checked_add(1) == Some(total) && first <= last).then_some((first, total))
}
