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

//! Two families' packs of one package have two names, and a run looks a
//! file up in its own family's packs and no other's.

use super::*;

#[test]
fn two_families_packs_of_one_package_have_two_names() {
    for (i, a) in ALL.into_iter().enumerate() {
        for b in ALL.into_iter().skip(i + 1) {
            let (at_a, at_b) = (pack_path(a, "vim"), pack_path(b, "vim"));
            assert_ne!(at_a, at_b, "{a:?} {b:?}");
            assert_ne!(trailer_of(&at_a), trailer_of(&at_b));
            assert_ne!(new_path(&at_a), new_path(&at_b));
            assert!(!is_packs_of(b, &at_a) && !is_packs_of(a, &at_b), "{a:?} {b:?}");
        }
        // Under the prefix capsule_vfs holds for a run (fdtable/held.rs).
        assert!(pack_path(a, "vim").starts_with(b"/linux-packs/"));
        assert!(is_packs_of(a, &pack_path(a, "vim")));
    }
}

#[test]
fn a_run_finds_a_packed_file_only_in_its_own_familys_packs() {
    let mut m = every_vim();
    put(&mut m, Family::Debian, "jq", b"/usr/bin/jq");
    // A name that reads as a path into another family's packs is still its own family's.
    put(&mut m, Family::Alpine, "deb/sh", b"/bin/sh");
    let found = |f: Family, file: &[u8]| loads(&m, f).iter().any(|p| p.as_slice() == file);
    assert!(found(Family::Debian, b"/usr/bin/jq"));
    assert!(!found(Family::Alpine, b"/usr/bin/jq") && !found(Family::Pacman, b"/usr/bin/jq"));
    assert!(!found(Family::Debian, b"/bin/sh"));
    assert_eq!(loads(&m, Family::Alpine), [b"/bin/sh".to_vec(), vim_of(Family::Alpine)]);
    assert_eq!(loads(&m, Family::Debian), [b"/usr/bin/jq".to_vec(), vim_of(Family::Debian)]);
    assert_eq!(loads(&m, Family::Pacman), [vim_of(Family::Pacman)]);
}

#[test]
fn a_trailer_or_a_left_over_new_is_no_familys_pack() {
    let at = pack_path(Family::Debian, "vim");
    for f in ALL {
        assert!(!is_packs_of(f, &trailer_of(&at)) && !is_packs_of(f, &new_path(&at)), "{f:?}");
    }
}
