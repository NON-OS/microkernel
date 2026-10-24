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

//! Through random_range, random_u32 and random_u64.

#![allow(dead_code, clippy::let_unit_value)]

include!("../src/mount.rs");

fn said() -> String {
    String::from_utf8(stand_in::SERIAL.lock().unwrap().clone()).unwrap()
}

#[test]
fn the_first_draw_before_seeding_names_its_caller_once() {
    stand_in::sources(true, true, true);
    let _ = crypto::rng::random_range(1000);
    assert_eq!(said(), "", "a seeded draw says nothing");
    stand_in::sources(false, true, false);
    let (_, at) = (crypto::rng::random_range(1000), line!());
    let first = format!("[RNG] pre-seed draw by {}:{at}\n", file!());
    let want = first;
    assert_eq!(said(), want);
    let _ = crypto::rng::random_range(1000);
    assert_eq!(said(), want, "each line is said once");
}
