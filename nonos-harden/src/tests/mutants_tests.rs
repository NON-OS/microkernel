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

//! The mutant list (mutants.json, run by mutants.py) stays true to the code:
//! each names a file of this crate, a change whose `old` text occurs there
//! exactly once, and the proof that catches it; so no mutant silently stops
//! applying when the code it breaks is edited.

use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[test]
fn every_mutant_applies_exactly_once_and_names_its_proof() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let list: Vec<Value> = serde_json::from_str(include_str!("../../mutants.json")).unwrap();
    assert!(list.len() >= 25, "only {} mutants", list.len());
    let mut names = BTreeSet::new();
    for m in &list {
        let field = |k: &str| m[k].as_str().unwrap_or_else(|| panic!("{m}: no {k}"));
        let (name, file, old) = (field("name"), field("file"), field("old"));
        assert!(names.insert(name), "{name} is listed twice");
        assert_ne!(old, field("new"), "{name} changes nothing");
        let src = fs::read_to_string(crate_dir.join(file)).unwrap();
        assert_eq!(src.matches(old).count(), 1, "{name}: `old` in {file}");
        assert!(field("caught_by").starts_with("tests::"), "{name} names no proof");
    }
}
