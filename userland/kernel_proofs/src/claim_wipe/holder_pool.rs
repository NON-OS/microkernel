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

//! The names a claimant may serve, for the holder tests, and the proof that
//! a record and a claimant match only by one fitting name: two names that
//! differ anywhere, in their last byte or only in length, never match, and
//! a side with no name matches nothing, not even another with none.

use super::holder_rule::{name_of, same, NAME_MAX};

/// The rule's own test of a name, restated: under the root, and fits.
pub(super) fn fits(n: &str) -> bool {
    n.starts_with("driver.") && n.len() <= NAME_MAX
}

/// Foreign names that sort first, a near miss of the root, the bare root,
/// two drivers, one name exactly as long as fits, two longer ones that
/// would sort first, and two that differ only in their last byte.
pub(super) fn pool() -> Vec<String> {
    let long = |c: &str, len: usize| format!("driver.{}", c.repeat(len - "driver.".len()));
    let mut all: Vec<String> =
        ["", "app.store", "Driver.a", "driverx.a", "driver.", "driver.e1000_0", "driver.bga0"]
            .map(String::from)
            .into();
    all.extend([long("b", NAME_MAX), long("a", NAME_MAX + 1), long("0", 4 * NAME_MAX)]);
    all.extend(["1", "2"].map(|c| long("c", NAME_MAX - 1) + c));
    all
}

#[test]
fn a_record_and_a_claimant_match_by_one_fitting_name_and_none_matches_nothing() {
    assert_eq!(NAME_MAX, 32, "a record holds at most this many bytes of the name");
    let mut names = pool();
    names.extend(["driver.bga0\0", "driver.bga"].map(String::from));
    let mut matched = 0;
    for a in &names {
        let na = name_of([a.as_str()]);
        assert_eq!(na.is_some(), fits(a), "{a:?}");
        assert!(!same(na, None) && !same(None, na), "{a:?} against none");
        for b in &names {
            let both = same(na, name_of([b.as_str()]));
            assert_eq!(both, fits(a) && a == b, "{a:?} {b:?}");
            matched += both as usize;
        }
    }
    assert_eq!(matched, names.iter().filter(|n| fits(n)).count());
    assert_eq!(matched, 8, "every fitting name matches itself and only itself");
}
