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

//! Each family's pack of one package is a name of its own: putting one in or
//! taking one out leaves the others, and a run of one family loads, and so
//! finds a file in, only its own family's packs.

use super::*;

const ALL: [Family; 3] = [Family::Alpine, Family::Debian, Family::Pacman];

/// What a run of `family` loads from the store: the packs its filter takes,
/// each only under the trailer minted over it.
fn loads(m: &Model, family: Family) -> Vec<Vec<u8>> {
    m.files.keys().filter(|at| is_packs_of(family, at)).filter_map(|at| served_at(m, at)).collect()
}

fn put(m: &mut Model, family: Family, package: &str, pack: &[u8]) {
    assert_eq!(swap(m, &pack_path(family, package), pack, &minted(pack)), Ok(()));
}

/// The one file each family's vim pack holds: a path naming the family.
fn vim_of(family: Family) -> Vec<u8> {
    format!("/usr/share/{family:?}/vim").into_bytes()
}

/// Every family's vim in.
fn every_vim() -> Model {
    let mut m = Model::default();
    for f in ALL {
        put(&mut m, f, "vim", &vim_of(f));
    }
    m
}

mod lookup;
mod out;
