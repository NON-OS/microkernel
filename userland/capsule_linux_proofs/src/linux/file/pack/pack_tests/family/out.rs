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

//! One family's pack put in, taken out or held open, and every other
//! family's pack of the same package as it was.

use super::*;

#[test]
fn taking_one_familys_pack_out_leaves_the_others() {
    for gone in ALL {
        let mut m = every_vim();
        assert_eq!(take(&mut m, &pack_path(gone, "vim")), Ok(()));
        assert_eq!(loads(&m, gone), Vec::<Vec<u8>>::new(), "{gone:?}");
        for f in ALL.into_iter().filter(|f| *f != gone) {
            assert_eq!(loads(&m, f), [vim_of(f)], "{gone:?} out, {f:?} stays");
        }
    }
}

#[test]
fn putting_one_familys_pack_in_leaves_the_others() {
    let mut m = every_vim();
    put(&mut m, Family::Debian, "vim", b"/usr/bin/vim.basic");
    assert_eq!(loads(&m, Family::Debian), [b"/usr/bin/vim.basic".to_vec()]);
    assert_eq!(loads(&m, Family::Alpine), [vim_of(Family::Alpine)]);
    assert_eq!(loads(&m, Family::Pacman), [vim_of(Family::Pacman)]);
}

#[test]
fn a_run_holding_its_familys_pack_holds_no_other_familys() {
    let debian = pack_path(Family::Debian, "vim");
    let mut m = Model { held: Some(debian.clone()), ..every_vim() };
    put(&mut m, Family::Alpine, "vim", b"/usr/bin/vim");
    assert_eq!(take(&mut m, &pack_path(Family::Pacman, "vim")), Ok(()));
    assert_eq!(take(&mut m, &debian), Err(Held::InUse));
    assert_eq!(loads(&m, Family::Debian), [vim_of(Family::Debian)]);
}
