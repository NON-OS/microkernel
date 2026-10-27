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

//! A map follows its file through a rename, its own or a directory's above
//! it; after an unlink, or a rename onto its file, no file of that name is
//! its, and a new one there is mapped as if it had never been.

use super::support::{map, writer, Entry};
use crate::linux::file::shared_maps::names::{moved, rename, unlink};
use crate::linux::file::shared_maps::rules::admits;

#[test]
fn a_path_moves_with_itself_or_a_directory_above_it() {
    assert_eq!(moved(b"/h/db/data.mdb", b"/h/db/data.mdb", b"/h/x"), Some(b"/h/x".to_vec()));
    assert_eq!(moved(b"/h/db/data.mdb", b"/h/db", b"/h/old"), Some(b"/h/old/data.mdb".to_vec()));
    assert_eq!(moved(b"/h/dbx/data.mdb", b"/h/db", b"/h/old"), None, "a name is not a prefix");
    assert_eq!(moved(b"/h/d", b"/h/db", b"/h/old"), None);
}

#[test]
fn a_rename_takes_the_maps_along_and_orphans_the_ones_it_replaced() {
    let mut all = vec![writer(map(1, 16, 1, "/h/db/a", 0)), map(2, 16, 1, "/h/new/a", 0)];
    all.push(map(3, 16, 1, "/h/other", 0));
    rename(&mut all, b"/h/db", b"/h/new");
    assert_eq!(all[0].path, b"/h/new/a".to_vec());
    assert!(!all[0].orphan, "a renamed file is still the map's");
    assert!(all[1].orphan, "the file the rename replaced is no longer anyone's");
    assert_eq!(all[2], map(3, 16, 1, "/h/other", 0), "another file is left alone");
}

#[test]
fn an_unlinked_file_keeps_its_maps_and_lends_its_name_to_no_new_file() {
    let mut all = vec![writer(map(1, 16, 2, "/h/f", 0)), map(2, 16, 1, "/h/g", 0)];
    unlink(&mut all, b"/h/f");
    assert!(all[0].orphan && !all[1].orphan);
    assert_eq!((all[0].at, all[0].span), (16 * super::support::P, 2 * super::support::P));
    assert!(!all[0].shows(b"/h/f"), "no write of the new /h/f reaches it");
    let again: Entry = writer(map(2, 32, 2, "/h/f", 0));
    assert_eq!(admits(&all, &again, false), Ok(()), "nor does it stand in a new map's way");
    rename(&mut all, b"/h/f", b"/h/z");
    assert_eq!(all[0].path, b"/h/f".to_vec(), "an orphan follows no rename");
}
