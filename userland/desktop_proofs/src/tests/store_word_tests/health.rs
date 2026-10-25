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

//! What the shell says of a store that loaded.

use crate::store_state::store_word::store_word;
use crate::store_state::NotifyLevel;
use nonos_app_skeleton::clients::vfs::store_was_read;

/* A store that loaded with one entry refused (9) is a disk that works: a
 * warning, never "corrupted". */
#[test]
fn one_refused_entry_is_a_warning_not_corruption() {
    let (text, level) = store_word(9).expect("said");
    assert!(level == NotifyLevel::Warn);
    assert!(!text.windows(9).any(|w| w == b"corrupted"));
    assert!(store_was_read(9), "and a disk was read");
}

#[test]
fn a_store_read_and_not_decoded_is_an_error() {
    for code in [3, 6] {
        let (_, level) = store_word(code).expect("said");
        assert!(level == NotifyLevel::Error, "code {code}");
    }
}

/* A healthy store, and codes vfs never latches at boot, say nothing. */
#[test]
fn a_healthy_store_says_nothing() {
    for code in [0, 8, 10, 11, 12] {
        assert!(store_word(code).is_none(), "code {code}");
    }
}
