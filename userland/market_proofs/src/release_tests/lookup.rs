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

//! Readiness and the release lookup resolve a request to the same release.
//! An empty id is the listing's default, its first release, in both: when
//! only readiness knew that, init was told a package was ready and then
//! found no release to pin, and refused every store install.

use alloc::string::String;

use super::index::index;
use crate::find_release::find_release;

#[test]
fn an_empty_release_id_is_the_first_release() {
    let got = find_release(&index(), "linux.jq", "").map(|(_, i, r)| (i, r.release_id.clone()));
    assert_eq!(got, Some((0, String::from("jq@1.7.1-r0"))));
}

#[test]
fn a_named_release_is_that_release() {
    let got = find_release(&index(), "linux.jq", "jq@1.6-r4").map(|(_, i, _)| i);
    assert_eq!(got, Some(1));
}

#[test]
fn an_unknown_listing_or_release_is_none() {
    assert!(find_release(&index(), "linux.nope", "").is_none());
    assert!(find_release(&index(), "linux.jq", "jq@0").is_none());
}
