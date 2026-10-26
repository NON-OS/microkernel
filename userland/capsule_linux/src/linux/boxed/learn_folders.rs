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
//! The fewest, narrowest folders that let every path a learn run wrote be
//! written: each path's own folder, or the path itself when its folder is one
//! of the run's own (so `/tmp/out` for `mkdir /tmp/out`, not all of `/tmp`);
//! then, while there are more than a file may name or they take more than
//! `room` bytes of the box word, each one's folder just under the run's own,
//! then the run's own folders themselves.

use alloc::vec::Vec;

use super::folders::under;
use super::preset::OWN;

/// The most `write` lines a `nonos-box 1` file has.
pub const WRITES_MAX: usize = 16;

pub fn folders(wrote: &[Vec<u8>], room: usize) -> Vec<Vec<u8>> {
    let mut out = collapse(wrote.iter().filter_map(|p| narrow(p)).collect());
    let size = |f: &[Vec<u8>]| f.iter().map(|f| f.len() + 1).sum::<usize>();
    let over = |f: &[Vec<u8>]| f.len() > WRITES_MAX || size(f) > room;
    for widen in [first_level as fn(&[u8]) -> Option<Vec<u8>>, own_root] {
        if over(&out) {
            out = collapse(out.iter().filter_map(|f| widen(f)).collect());
        }
    }
    out
}

/// The run's own folder `path` is in, if any.
fn root_of(path: &[u8]) -> Option<&'static [u8]> {
    OWN.iter().copied().find(|o| under(path, o))
}

fn narrow(path: &[u8]) -> Option<Vec<u8>> {
    let root = root_of(path)?;
    let parent = &path[..path.iter().rposition(|b| *b == b'/')?];
    Some(match parent.len() <= root.len() {
        true => path.to_vec(),
        false => parent.to_vec(),
    })
}

fn first_level(f: &[u8]) -> Option<Vec<u8>> {
    let root = root_of(f)?;
    let below = f.get(root.len() + 1..).unwrap_or_default();
    let end = below.iter().position(|b| *b == b'/').map_or(f.len(), |e| root.len() + 1 + e);
    Some(f[..end].to_vec())
}

fn own_root(f: &[u8]) -> Option<Vec<u8>> {
    root_of(f).map(<[u8]>::to_vec)
}

/// Sorted, once each, and none under another.
fn collapse(mut all: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    all.sort();
    all.dedup();
    all.iter().filter(|f| !all.iter().any(|o| o != *f && under(f, o))).cloned().collect()
}
