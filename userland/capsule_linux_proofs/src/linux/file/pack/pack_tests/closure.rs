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

//! A program's closure as the install keeps it, and the packs a run of it
//! loads: only its closure's, or every one of its family's with none.

use super::super::closure::{decode, encode, serves, MAX_BYTES, MAX_NAMES};
use super::*;

fn names(of: &[&str]) -> Vec<String> {
    of.iter().map(|s| s.to_string()).collect()
}

/// The packages whose packs, among `listed`, a run with `closure` loads.
fn loaded(listed: &[Vec<u8>], closure: Option<&[String]>) -> Vec<String> {
    let of = |at: &Vec<u8>| package_of(Family::Alpine, at).map(|p| p.to_vec());
    let kept = listed.iter().filter_map(of).filter(|p| serves(closure, p));
    kept.map(|p| String::from_utf8(p).unwrap()).collect()
}

#[test]
fn a_closure_reads_back_as_it_was_kept() {
    let c = names(&["keepassxc", "qt6-qtbase", "musl"]);
    assert_eq!(encode(&c).as_deref(), Some(b"keepassxc\nqt6-qtbase\nmusl\n".as_slice()));
    assert_eq!(decode(&encode(&c).unwrap()), Some(c));
    assert_eq!(decode(b""), Some(Vec::new()));
}

#[test]
fn a_closure_past_its_bounds_or_cut_short_is_none() {
    let many: Vec<String> = (0..=MAX_NAMES).map(|i| format!("p{i}")).collect();
    assert_eq!(encode(&many), None);
    assert_eq!(decode(many.join("\n").as_bytes()), None, "cut short of its last line");
    assert_eq!(decode(format!("{}\n", many.join("\n")).as_bytes()), None, "past MAX_NAMES");
    assert_eq!(encode(&[String::from("x").repeat(MAX_BYTES as usize)]), None);
    assert_eq!(encode(&names(&["two\nlines"])), None);
    assert_eq!(encode(&names(&[""])), None);
    assert_eq!(decode(&vec![b'\n'; MAX_BYTES as usize + 1]), None);
}

#[test]
fn a_run_loads_only_its_closures_packs() {
    let a = Family::Alpine;
    let mut listed: Vec<Vec<u8>> = ["swayimg", "keepassxc", "qt6-qtbase", "musl", "authenticator"]
        .iter()
        .map(|p| pack_path(a, p))
        .collect();
    listed.push(trailer_of(&pack_path(a, "musl")));
    listed.push(pack_path(Family::Debian, "musl"));
    let c = names(&["keepassxc", "qt6-qtbase", "musl", "not-installed"]);
    assert_eq!(loaded(&listed, Some(&c)), ["keepassxc", "qt6-qtbase", "musl"]);
}

#[test]
fn a_run_with_no_closure_loads_every_pack_of_its_family() {
    let a = Family::Alpine;
    let mut listed: Vec<Vec<u8>> = ["swayimg", "musl"].iter().map(|p| pack_path(a, p)).collect();
    listed.push(pack_path(Family::Pacman, "glibc"));
    assert_eq!(loaded(&listed, None), ["swayimg", "musl"]);
}
