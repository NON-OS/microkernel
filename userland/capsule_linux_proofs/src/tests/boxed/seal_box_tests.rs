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

//! In a box, slot 0 binds the app to the box it was held in, so a receipt
//! names the box; a BusyBox run in a box is noted and sealed too.

use crate::boxed::hash::box_hash;
use crate::boxed::preset::parse;
use crate::boxed::seal_slot::{slot0, BOXED_DOMAIN};

#[test]
fn in_a_box_the_note_is_the_app_and_the_box() {
    let (strict, offline) =
        (box_hash(&parse(b"strict").unwrap()), box_hash(&parse(b"offline").unwrap()));
    for app in [[0; 32], [7; 32]] {
        let want = *blake3::Hasher::new()
            .update(BOXED_DOMAIN)
            .update(&app)
            .update(&strict)
            .finalize()
            .as_bytes();
        assert_eq!(slot0(&app, Some(&strict)), Some(want));
        assert_ne!(slot0(&app, Some(&strict)), slot0(&app, Some(&offline)), "another box");
        assert_ne!(slot0(&app, Some(&strict)), Some(app), "never the bare app_id");
    }
    assert_ne!(slot0(&[0; 32], Some(&strict)), Some([0; 32]), "BusyBox in a box is noted");
    assert_ne!(slot0(&[1; 32], Some(&strict)), slot0(&[2; 32], Some(&strict)), "another app");
    assert_eq!(BOXED_DOMAIN, b"NONOS.agentproof.app.v1");
}
