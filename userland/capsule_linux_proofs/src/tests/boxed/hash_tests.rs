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

//! One canonical hash for each box: stable, whatever order it was named in,
//! and different for every box that allows something different.

use crate::boxed::hash::box_hash;
use crate::boxed::preset::parse;
use crate::boxed::report::hex;

fn of(word: &[u8]) -> String {
    hex(&box_hash(&parse(word).unwrap()))
}

/* Pinned: a preset's hash changes only when what it allows changes. */
pub(super) const STRICT: &str = "5811b081f826f154ce1ac5a7b558bff858ae0bacc27ad2db9d2e5f84ce05323a";
const OFFLINE: &str = "630d02b9ab8f611ff1d39ba26c29be18566995b5663fd25c0965c47cb4a66776";

#[test]
fn the_presets_hash_as_pinned() {
    assert_eq!(of(b"strict"), STRICT);
    assert_eq!(of(b"offline"), OFFLINE);
}

#[test]
fn an_allow_box_hashes_alike_in_any_order() {
    assert_eq!(
        of(b"allow:/tmp/b,net:example.com:443,/tmp/a,net:10.0.0.1:80"),
        of(b"allow:net:10.0.0.1:80,/tmp/a,/tmp/a,/tmp/b,net:EXAMPLE.com:443"),
    );
}

#[test]
fn every_difference_changes_the_hash() {
    let all = [
        of(b"strict"),
        of(b"offline"),
        of(b"allow:/tmp"),
        of(b"allow:/tmp/a"),
        of(b"allow:/tmp/a,/tmp/b"),
        of(b"allow:/tmp/a,net:10.0.0.1:80"),
        of(b"allow:/tmp/a,net:10.0.0.1:81"),
        of(b"allow:/tmp/a,net:10.0.0.2:80"),
        of(b"allow:/tmp/a,net:example.com:80"),
    ];
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert_ne!(a, b);
        }
    }
}
