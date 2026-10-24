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

//! Through the byte API's three layers, with the hardware answering.

#![allow(dead_code, clippy::let_unit_value)]

include!("../src/mount.rs");

fn said() -> String {
    String::from_utf8(stand_in::SERIAL.lock().unwrap().clone()).unwrap()
}

#[test]
fn the_first_draw_before_seeding_names_its_caller_once() {
    stand_in::sources(true, true, true);
    let mut buf = [0u8; 8];
    let _ = crypto::random_api::get_bytes(&mut buf);
    assert_eq!(said(), "", "a seeded draw says nothing");
    stand_in::sources(false, true, false);
    let (_, at) = (crypto::random_api::get_bytes(&mut buf), line!());
    let first = format!("[RNG] pre-seed draw by {}:{at}\n", file!());
    let want = first;
    assert_eq!(said(), want);
    let _ = crypto::random_api::get_bytes(&mut buf);
    assert_eq!(said(), want, "each line is said once");
}
