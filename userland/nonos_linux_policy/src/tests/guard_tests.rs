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

//! An empty vendor table is a dev build's, never a shipping one's.

#[path = "../../build/guard.rs"]
mod guard;

use guard::{may_build, Build};

const NONOS: &str = "x86_64-nonos-user";
const HOST: &str = "x86_64-unknown-linux-gnu";

fn build(target: &'static str, profile: &'static str, in_flake: bool) -> Build<'static> {
    Build { target, host: HOST, profile, in_flake }
}

#[test]
fn a_shipping_build_needs_the_userland_and_a_row() {
    for b in [build(NONOS, "release", false), build(NONOS, "debug", true)] {
        assert!(b.ships());
        assert!(may_build(&b, false, 0).unwrap_err().contains("NONOS_LINUX_USERLAND"));
        assert!(may_build(&b, true, 0).unwrap_err().contains("names no program"));
        assert_eq!(may_build(&b, true, 3), Ok(()));
    }
}

#[test]
fn a_dev_build_and_every_host_test_keep_the_empty_table() {
    for b in
        [build(NONOS, "debug", false), build(HOST, "release", true), build(HOST, "debug", true)]
    {
        assert!(!b.ships());
        assert_eq!(may_build(&b, false, 0), Ok(()));
    }
}
