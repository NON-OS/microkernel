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

//! A name has one spelling in a list, and a name never stands in for an
//! address.

use super::dests_write::{entry, name, raw};
use crate::dests::{check, DestsError as E};

fn takes(n: &str) -> bool {
    check(&raw(&[name(n, (443, 443))])).is_ok()
}

#[test]
fn lowercase_dotted_labels_are_names() {
    for n in ["example.org", "a.b-c.example", "xn--bcher-kva.example", "3com.com", "localhost"] {
        assert!(takes(n), "{n}");
    }
    let longest = ["a".repeat(63), "b".repeat(63), "c".repeat(63), "d".repeat(61)].join(".");
    assert_eq!(longest.len(), 253);
    assert!(takes(&longest));
}

#[test]
fn every_other_spelling_is_refused() {
    let label64 = format!("{}.org", "a".repeat(64));
    let long = ["a".repeat(63), "b".repeat(63), "c".repeat(63), "d".repeat(62)].join(".");
    for n in [
        "",
        "Example.org",
        "example.org.",
        ".example.org",
        "a..org",
        "-a.org",
        "a-.org",
        "a_b.org",
        "a b.org",
        "1.2.3.4",
        "host.123",
        &label64,
        &long,
    ] {
        assert!(!takes(n), "{n:?}");
        assert_eq!(check(&raw(&[name(n, (443, 443))])).err(), Some(E::Name), "{n:?}");
    }
}

#[test]
fn a_name_has_no_prefix() {
    assert_eq!(check(&raw(&[entry(b'N', 8, (1, 1), b"example.org")])), Err(E::Name));
}
