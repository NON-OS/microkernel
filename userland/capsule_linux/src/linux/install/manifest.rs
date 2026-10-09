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

//! What one package put into the store, kept beside its program record so it
//! can be taken out again: each file it wrote, and each link it added to the
//! link table. One line each, `f <path>` or `l <path>`; a path holding a
//! newline cannot be written in it, and the link table refuses those too.
//! Pure, so capsule_linux_proofs holds the format and the removal rules.

use alloc::vec::Vec;

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Manifest {
    pub files: Vec<Vec<u8>>,
    pub links: Vec<Vec<u8>>,
}

pub fn encode(m: &Manifest) -> Vec<u8> {
    let mut out = Vec::new();
    let tagged = m.files.iter().map(|p| (b'f', p)).chain(m.links.iter().map(|p| (b'l', p)));
    for (tag, path) in tagged.filter(|(_, p)| !p.is_empty() && !p.contains(&b'\n')) {
        out.push(tag);
        out.push(b' ');
        out.extend_from_slice(path);
        out.push(b'\n');
    }
    out
}

/// Every well formed line; anything else is skipped rather than guessed at.
pub fn decode(bytes: &[u8]) -> Manifest {
    let mut m = Manifest::default();
    for line in bytes.split(|b| *b == b'\n') {
        match line {
            [b'f', b' ', path @ ..] if !path.is_empty() => m.files.push(path.to_vec()),
            [b'l', b' ', path @ ..] if !path.is_empty() => m.links.push(path.to_vec()),
            _ => {}
        }
    }
    m
}

/// `mine` less every path another installed package also lists: a file two
/// packages both wrote stays until neither needs it.
pub fn only_mine(
    mine: &[Vec<u8>],
    others: &[Manifest],
    pick: fn(&Manifest) -> &[Vec<u8>],
) -> Vec<Vec<u8>> {
    mine.iter().filter(|p| !others.iter().any(|o| pick(o).contains(p))).cloned().collect()
}

/// The link table without the lines for `paths`, and how many went.
pub fn drop_links(table: &[u8], paths: &[Vec<u8>]) -> (Vec<u8>, usize) {
    let mut out = Vec::with_capacity(table.len());
    let mut dropped = 0usize;
    for line in table.split(|b| *b == b'\n').filter(|l| !l.is_empty()) {
        let path = line.split(|b| *b == b' ').next().unwrap_or(line);
        if paths.iter().any(|p| p.as_slice() == path) {
            dropped += 1;
            continue;
        }
        out.extend_from_slice(line);
        out.push(b'\n');
    }
    (out, dropped)
}
