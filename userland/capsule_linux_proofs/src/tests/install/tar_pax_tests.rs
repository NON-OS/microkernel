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

//! Pax records (tar_pax.rs): each refusal stops the reading and keeps what
//! the records before it said; a record with no `=` is passed over.

use crate::install::tar_pax::{read, Overrides};

fn of(body: &[u8]) -> (Option<Vec<u8>>, Option<Vec<u8>>) {
    let mut o = Overrides::default();
    read(body, &mut o);
    (o.path, o.link)
}

/// A record whose length counts the whole of it, its own digits included.
fn rec(kv: &str) -> Vec<u8> {
    let body = kv.len() + 2;
    let len = (1..).map(|d| body + d).find(|n| n.to_string().len() + body == *n).unwrap();
    format!("{len} {kv}\n").into_bytes()
}

#[test]
fn the_helper_counts_as_pax_does() {
    assert_eq!(rec("path=usr/a"), b"14 path=usr/a\n");
    assert_eq!(rec("a=b"), b"6 a=b\n");
}

#[test]
fn path_and_linkpath_are_taken_and_other_keys_are_not() {
    let body = [rec("path=usr/a"), rec("linkpath=lib"), rec("mtime=1700000000")].concat();
    assert_eq!(of(&body), (Some(b"usr/a".to_vec()), Some(b"lib".to_vec())));
}

#[test]
fn a_record_with_no_equals_is_passed_over() {
    assert_eq!(of(&[rec("nokey"), rec("path=usr/a")].concat()).0, Some(b"usr/a".to_vec()));
}

#[test]
fn each_malformed_record_stops_the_reading_and_keeps_what_came_before() {
    for bad in [
        &b"no length at all"[..],
        b"nospace",
        b"1x path=b\n",
        b" path=b\n",
        b"99 path=b\n",
        b"2 path=b\n",
        b"99999999999999999999999 path=b\n",
    ] {
        assert_eq!(of(bad), (None, None), "{:?}", core::str::from_utf8(bad));
        assert_eq!(of(&[&rec("path=usr/a")[..], bad].concat()).0, Some(b"usr/a".to_vec()));
    }
}
