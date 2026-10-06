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

//! The root a slot's path folds to, for a caller that must name the root it
//! proves under rather than check against one it was given.

use crate::leaf::{leaf, Kind};
use crate::poseidon::Poseidon;
use crate::trailer::{digest_to_bytes, parse};
use crate::verify::fold;

/// The root `trailer`'s path folds to from the leaf of `kind` over `ctx`, the
/// fold `verify` does. `None` for the pad kind, a context with no leaf, or a
/// path that is malformed or carries a non-canonical sibling.
pub fn root_of(depth: usize, kind: Kind, ctx: &[u8], trailer: &[u8]) -> Option<[u8; 32]> {
    if kind == Kind::Pad {
        return None;
    }
    let path = parse(trailer, depth)?;
    let h = Poseidon::new();
    let start = leaf(&h, kind, ctx)?;
    Some(digest_to_bytes(&fold(&h, start, &path)?))
}
