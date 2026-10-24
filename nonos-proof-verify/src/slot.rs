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

//! Which slot a trailer opens. The path's direction bits, from the leaf up,
//! are the slot's index, bit k at level k, the way `Tree::trailer` writes
//! them. This only says which transcript slot to build the context from; the
//! check then decides, so a trailer that lies about its slot is refused.

use nonos_attest_path::{parse_v4, Kind, MAGIC, MAX_PROOF_V4};

use crate::check::DEPTH;

const PATH_LEN: usize = 9 + DEPTH * 32 + DEPTH.div_ceil(8);

/// What a trailer says it is: a v4 trailer's kind, or none for a development
/// trailer (the path alone), and the slot its path opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Opens {
    pub kind: Option<Kind>,
    pub index: usize,
}

/// Read the slot a trailer opens, or `None` when it is neither a v4 trailer of
/// a proven kind nor a development path of the gates' depth.
pub fn opens(t: &[u8]) -> Option<Opens> {
    if t.starts_with(&MAGIC) {
        return Some(Opens { kind: None, index: index(t)? });
    }
    [Kind::Kernel, Kind::Capsule, Kind::Bootloader].into_iter().find_map(|k| {
        let v = parse_v4(t, k, MAX_PROOF_V4)?;
        Some(Opens { kind: Some(k), index: index(v.path)? })
    })
}

fn index(path: &[u8]) -> Option<usize> {
    if path.len() != PATH_LEN || *path.get(8)? as usize != DEPTH {
        return None;
    }
    let dirs = &path[9 + DEPTH * 32..];
    Some(
        (0..DEPTH)
            .filter(|&k| (usize::from(dirs[k / 8]) >> (k % 8)) & 1 == 1)
            .map(|k| 1 << k)
            .sum(),
    )
}
