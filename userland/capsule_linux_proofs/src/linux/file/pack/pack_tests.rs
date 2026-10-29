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

//! A pack swapped in a model store that stops dead after any step, as a
//! crash does: what it serves then is the old pack, nothing, or the new.
//! And each family's packs, apart from every other family's (family.rs).

use super::paths::{pack_path, package_of, trailer_of};
use super::shelf::Held;
use super::swap::{new_path, swap, take};
use crate::family::Family;
use model::Model;

mod model;

/// The pack the order is proved on: Alpine's vim.
fn vim() -> Vec<u8> {
    pack_path(Family::Alpine, "vim")
}

/// Whether `at` is one of `family`'s packs.
fn is_packs_of(family: Family, at: &[u8]) -> bool {
    package_of(family, at).is_some()
}

fn minted(pack: &[u8]) -> Vec<u8> {
    [b"minted over ".as_slice(), pack].concat()
}

/// The pack at `at` as a run would load it: only under the trailer minted
/// over it.
fn served_at(m: &Model, at: &[u8]) -> Option<Vec<u8>> {
    let pack = m.files.get(at)?;
    (m.files.get(&trailer_of(at)) == Some(&minted(pack))).then(|| pack.clone())
}

fn served(m: &Model) -> Option<Vec<u8>> {
    served_at(m, &vim())
}

fn with_old() -> Model {
    let mut m = Model::default();
    assert_eq!(swap(&mut m, &vim(), b"old", &minted(b"old")), Ok(()));
    m
}

mod closure;
mod family;
mod held;
mod order;
