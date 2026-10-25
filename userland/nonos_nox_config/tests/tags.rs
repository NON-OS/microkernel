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

mod keccak;

use keccak::keccak;
use nonos_nox_config::{TAGS, UNITS};

#[test]
fn every_tag_is_its_label() {
    for (label, tag) in TAGS {
        assert_eq!(keccak(label.as_bytes()), tag, "{label}");
        assert!(label.starts_with("NONOS.nox.") && label.ends_with(".v1"), "{label}");
    }
}

#[test]
fn every_unit_is_its_label() {
    for (label, unit) in UNITS {
        assert_eq!(keccak(label.as_bytes()), unit, "{label}");
    }
}

#[test]
fn no_two_words_collide() {
    let mut all: Vec<[u8; 32]> = TAGS.iter().chain(UNITS.iter()).map(|(_, w)| *w).collect();
    all.sort();
    all.dedup();
    assert_eq!(all.len(), TAGS.len() + UNITS.len());
}
