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

//! `market list`'s rows as pacman prints a search: `linux/htop  htop` and the
//! listing's standing in brackets, its colour saying the same thing the word
//! does; a file gets the words with no colour.

use alloc::vec::Vec;

/// The heading: `:: 12 listings`.
pub fn heading(count: usize) -> (Vec<u8>, Vec<u8>) {
    let mut n = [0u8; 20];
    let k = crate::term::util::format_u64(count as u64, &mut n);
    let word: &[u8] = if count == 1 { b" listing" } else { b" listings" };
    let plain = [b":: ", &n[..k], word].concat();
    let styled = [b"\x1b[1;34m::\x1b[0m \x1b[1m", &n[..k], word, b"\x1b[0m"].concat();
    (plain, styled)
}

/// The colour a standing is shown in: green installed, cyan ready, yellow
/// while it moves, red failed, and dim when it cannot install here.
pub fn tone(standing: &str) -> &'static [u8] {
    match standing {
        "installed" => b"\x1b[1;32m",
        "ready" => b"\x1b[1;36m",
        "installing" | "removing" => b"\x1b[1;33m",
        "failed" => b"\x1b[1;31m",
        _ => b"\x1b[2m",
    }
}

/// One listing: its id split at the first dot into the repository and the
/// rest, padded to `width` cells, its name, and its standing.
pub fn row(id: &[u8], name: &[u8], standing: &str, width: usize) -> (Vec<u8>, Vec<u8>) {
    let (repo, rest) = match id.iter().position(|&b| b == b'.') {
        Some(i) => (&id[..i], &id[i + 1..]),
        None => (&b""[..], id),
    };
    let shown = if repo.is_empty() { rest.len() } else { repo.len() + 1 + rest.len() };
    let pad = alloc::vec![b' '; width.saturating_sub(shown) + 2];
    let slash: &[u8] = if repo.is_empty() { b"" } else { b"/" };
    let tag = [b"[", standing.as_bytes(), b"]"].concat();
    let plain = [repo, slash, rest, &pad, name, b"  ", &tag].concat();
    let styled = [
        b"\x1b[1;35m",
        repo,
        b"\x1b[0m",
        slash,
        b"\x1b[1m",
        rest,
        b"\x1b[0m",
        &pad,
        name,
        b"  ",
        tone(standing),
        &tag,
        b"\x1b[0m",
    ]
    .concat();
    (plain, styled)
}
