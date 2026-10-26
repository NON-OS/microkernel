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

//! What the panel shows for each saved-network error is a sentence of its
//! own, and the record is kept inside the directory the store path makes.

use super::error::SavedError::{self, *};
use super::file::{DIR, PATH};

#[test]
fn every_saved_error_has_words_of_its_own() {
    let all: [SavedError; 9] =
        [NoStore, NotKept, NoTpm, BootChanged, KeyFailed, Unreadable, Damaged, NotSavable, Vfs("")];
    let mut words: Vec<&str> = all.iter().map(|e| e.text()).collect();
    assert!(words[..8].iter().all(|w| !w.is_empty()));
    words.sort();
    words.dedup();
    assert_eq!(words.len(), all.len(), "no two errors read alike");
    assert_eq!(Vfs("refused").text(), "refused", "the vfs service's own reason");
}

#[test]
fn the_record_lives_in_the_directory_the_store_makes() {
    assert_eq!((&PATH[..DIR.len()], PATH[DIR.len()]), (DIR, b'/'));
}
