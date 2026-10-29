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

//! The installed packages' paths kept for one install answer as the record
//! scan they replace (member/conflict.rs), and stay true as records are
//! written again or taken out (member/held_map.rs).

use crate::install::manifest::Manifest;
use crate::install::member::conflict::held_by_another;
use crate::install::member::held_map::Held;

fn m(files: &[&str], links: &[&str]) -> Manifest {
    let v = |s: &[&str]| s.iter().map(|p| p.as_bytes().to_vec()).collect();
    Manifest { files: v(files), links: v(links) }
}

fn paths(s: &[&str]) -> Vec<Vec<u8>> {
    s.iter().map(|p| p.as_bytes().to_vec()).collect()
}

fn records() -> Vec<(&'static str, Manifest)> {
    vec![
        ("icu-data-full", m(&["/usr/share/icu/74.2/icudt74l.dat"], &[])),
        ("qt6-qtbase", m(&["/usr/lib/libQt6Core.so.6.7.2"], &["/usr/lib/libQt6Core.so.6"])),
        ("shared", m(&["/usr/share/both"], &[])),
        ("also", m(&["/usr/share/both"], &[])),
    ]
}

#[test]
fn it_answers_as_the_scan_of_every_other_record() {
    let mut held = Held::default();
    records().iter().for_each(|(n, r)| held.keep(n, r));
    let asks = ["/usr/share/icu/74.2/icudt74l.dat", "/usr/lib/libQt6Core.so.6", "/usr/share/both"];
    for (name, _) in records() {
        let others: Vec<_> = records().into_iter().filter(|r| r.0 != name).map(|r| r.1).collect();
        for mine in asks.map(|a| paths(&[a, "/usr/bin/new"])) {
            let (kept, scan) = (held.held_by_another(&mine, name), held_by_another(&mine, &others));
            assert_eq!(kept, scan, "{name}");
        }
    }
}

#[test]
fn a_record_kept_again_lets_go_of_what_it_held() {
    let mut held = Held::default();
    held.keep("app", &m(&["/usr/bin/old", "/usr/bin/kept"], &[]));
    held.keep("app", &m(&["/usr/bin/kept"], &[]));
    assert_eq!(held.held_by_another(&paths(&["/usr/bin/old"]), "other"), None, "let go");
    assert!(held.held_by_another(&paths(&["/usr/bin/kept"]), "other").is_some());
    assert_eq!(held.held_by_another(&paths(&["/usr/bin/kept"]), "app"), None, "its own");
}

#[test]
fn a_record_taken_out_holds_nothing() {
    let mut held = Held::default();
    held.keep("partial", &m(&["/usr/lib/a"], &["/usr/lib/b"]));
    held.keep("partial", &Manifest::default());
    assert_eq!(held.held_by_another(&paths(&["/usr/lib/a", "/usr/lib/b"]), "next"), None);
}
