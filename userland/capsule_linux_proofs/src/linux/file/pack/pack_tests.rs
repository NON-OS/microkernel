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

use super::paths::{pack_path, trailer_path};
use super::shelf::Held;
use super::swap::{new_path, swap, take};
use model::Model;

mod model;

fn minted(pack: &[u8]) -> Vec<u8> {
    [b"minted over ".as_slice(), pack].concat()
}

/// The pack a run would load: only one under the trailer minted over it.
fn served(m: &Model) -> Option<Vec<u8>> {
    let pack = m.files.get(&pack_path("vim"))?;
    (m.files.get(&trailer_path("vim")) == Some(&minted(pack))).then(|| pack.clone())
}

fn with_old() -> Model {
    let mut m = Model::default();
    assert_eq!(swap(&mut m, "vim", b"old", &minted(b"old")), Ok(()));
    m
}

mod order;
