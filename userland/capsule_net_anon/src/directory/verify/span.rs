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

//! The byte range an authority certificate signature covers.

const START: &[u8] = b"dir-key-certificate-version";
const END: &[u8] = b"\ndir-key-certification";

/*
 * router_get_hash_impl in authcert_parse.c, with this start, this end and a
 * terminator of '\n'. The consensus boundary uses the same helper with ' '
 * instead, so the two ranges end differently by one keyword's worth of
 * whitespace. Using the consensus rule here yields a digest that matches no
 * certificate.
 */
pub fn cert_range(body: &[u8]) -> Option<(usize, usize)> {
    let start = find(body, START, 0)?;
    let marker = find(body, END, start + START.len())?;
    let after = marker + END.len();
    let newline = body.get(after..)?.iter().position(|b| *b == b'\n')?;
    Some((start, after + newline + 1))
}

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    let last = haystack.len() - needle.len();
    (from..=last).find(|at| &haystack[*at..*at + needle.len()] == needle)
}
