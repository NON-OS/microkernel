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

//! The heap is bound before anything runs: the display server allocates
//! from its first request on, and an unbound heap fails the first allocation
//! after the mode set ("memory allocation of 64 bytes failed").

const MAIN: &str = include_str!("../../../capsule_driver_bga/src/main.rs");

#[test]
fn the_heap_is_bound_before_the_driver_starts() {
    let entry = MAIN.find("extern \"C\" fn _start()").expect("an entry point");
    let bound = MAIN[entry..].find("if heap_init().is_err() {").expect("heap_init at entry");
    let started = MAIN[entry..].find("start_driver(").expect("start_driver");
    assert!(bound < started, "heap_init must run before start_driver");
}

#[test]
fn a_heap_that_cannot_be_bound_ends_the_capsule() {
    let at = MAIN.find("if heap_init().is_err() {").expect("heap_init at entry");
    assert!(MAIN[at..]
        .trim_start_matches("if heap_init().is_err() {")
        .trim_start()
        .starts_with("mk_exit(EXIT_HEAP_INIT);"));
}
