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

//! An app finds the desktop's peers when it is first opened, not at spawn:
//! a peer still missing then costs that open and nothing more, the next open
//! looks again, and peers once found are kept rather than looked up again.

use core::cell::Cell;

use super::open_peers::open_peers;

#[test]
fn a_missing_peer_leaves_the_app_to_ask_again() {
    let mut cache: Option<u32> = None;
    assert_eq!(open_peers(&mut cache, || None), None);
    assert_eq!(cache, None);
    assert_eq!(open_peers(&mut cache, || Some(7)), Some(&7));
}

#[test]
fn peers_found_once_are_kept() {
    let mut cache: Option<u32> = None;
    let lookups = Cell::new(0u32);
    let look = || {
        lookups.set(lookups.get() + 1);
        Some(7)
    };
    assert_eq!(open_peers(&mut cache, look), Some(&7));
    assert_eq!(open_peers(&mut cache, look), Some(&7));
    assert_eq!(open_peers(&mut cache, || None), Some(&7));
    assert_eq!(lookups.get(), 1);
}

#[test]
fn each_failed_open_looks_once() {
    let mut cache: Option<u32> = None;
    let lookups = Cell::new(0u32);
    for _ in 0..3 {
        let _ = open_peers(&mut cache, || {
            lookups.set(lookups.get() + 1);
            None
        });
    }
    assert_eq!(lookups.get(), 3);
    assert_eq!(cache, None);
}
