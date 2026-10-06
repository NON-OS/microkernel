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

use alloc::string::String;
use alloc::vec::Vec;

use super::evict::victim;

/// (listing, finished, when recorded)
fn table(rows: &[(&str, bool, u64)]) -> Vec<(String, bool, u64)> {
    rows.iter().map(|(k, f, at)| (String::from(*k), *f, *at)).collect()
}

fn pick(rows: &[(String, bool, u64)]) -> Option<&str> {
    victim(rows.iter().map(|(k, f, at)| (k, *f, *at))).map(String::as_str)
}

#[test]
fn the_oldest_finished_install_gives_way() {
    let rows = table(&[("linux.b", true, 7), ("linux.a", true, 3), ("linux.c", true, 9)]);
    assert_eq!(pick(&rows), Some("linux.a"));
}

#[test]
fn an_install_still_moving_never_gives_way_however_old() {
    let rows = table(&[("linux.old", false, 0), ("linux.done", true, 5), ("linux.new", false, 9)]);
    assert_eq!(pick(&rows), Some("linux.done"));
}

#[test]
fn a_table_of_installs_all_moving_has_no_room() {
    let rows = table(&[("linux.a", false, 1), ("linux.b", false, 2)]);
    assert_eq!(pick(&rows), None);
    assert_eq!(pick(&[]), None);
}

#[test]
fn a_full_table_of_finished_installs_is_emptied_oldest_first() {
    let mut rows: Vec<(String, bool, u64)> =
        (0..32u64).map(|i| (alloc::format!("linux.p{i}"), i % 3 != 0, 100 - i)).collect();
    let mut order = Vec::new();
    while let Some(k) = pick(&rows).map(String::from) {
        let at = rows.iter().position(|(r, _, _)| *r == k).unwrap();
        let (_, finished, when) = rows.remove(at);
        assert!(finished);
        order.push(when);
    }
    // Every finished one went, oldest first; every moving one stayed.
    assert!(order.windows(2).all(|w| w[0] < w[1]));
    assert!(rows.iter().all(|(_, finished, _)| !finished));
    assert_eq!(rows.len(), 11);
}
