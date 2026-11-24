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

//! The `[LINUX-TIME] start` line: every stage in order, `-` for one never
//! reached, nothing that names a program or a destination; and the marks
//! sit where the launch passes those stages.

#[path = "../../../capsule_linux/src/linux/start_time/line.rs"]
mod line;

use line::{line, WORDS};

const MOD: &str = include_str!("../../../capsule_linux/src/linux/start_time/mod.rs");
const START: &str = include_str!("../../../capsule_linux/src/linux/start.rs");
const START_GUEST: &str = include_str!("../../../capsule_linux/src/linux/start_guest.rs");
const ASK: &str = include_str!("../../../capsule_linux/src/linux/net/grant/ask.rs");

#[test]
fn a_full_run_says_every_stage_in_order() {
    let at = [13, 31, 32, 41, 56, 71, 73, 301, 321, 401, 1201];
    let said = line(5_627_979, &at, 12, 1500, "done");
    assert_eq!(
        said,
        "[LINUX-TIME] start up 5627979 ms read 12 grant 30 prep 31 family 40 proved 55 \
         loaded 70 running 72 libs 300 out 320 connect 400 byte 1200 end 1500 mapped 12 done\n"
    );
}

#[test]
fn a_stage_never_reached_is_a_dash_and_a_stop_says_so() {
    let mut at = [0u64; 11];
    at[0] = 1;
    at[1] = 9;
    let said = line(7, &at, 0, 9, "stopped");
    assert!(said.contains(" read 0 grant 8 prep - family - "), "{said}");
    assert!(said.ends_with(" connect - byte - end 9 mapped 0 stopped\n"), "{said}");
}

#[test]
fn the_slots_are_the_words_in_order() {
    let names = [
        "READ", "GRANT", "PREP", "FAMILY", "PROVED", "LOADED", "RUNNING", "LIBS", "OUT",
        "CONNECT", "BYTE",
    ];
    for (i, name) in names.iter().enumerate() {
        assert!(MOD.contains(&format!("pub const {name}: usize = {i};")), "{name} is not {i}");
    }
    assert!(MOD.contains(&format!("pub const STAGES: usize = {};", WORDS.len())));
}

#[test]
fn the_launch_marks_its_stages_where_it_passes_them() {
    let at = |src: &str, needle: &str| src.find(needle).unwrap_or_else(|| panic!("{needle}"));
    let marks = [
        "start_time::begin();",
        "start_time::reached(super::start_time::READ);",
        "start_time::reached(super::start_time::PREP);",
        "start_time::reached(super::start_time::FAMILY);",
        "start_time::reached(super::start_time::RUNNING);",
        "start_time::say(\"done\");",
    ];
    let places: Vec<usize> = marks.iter().map(|m| at(START, m)).collect();
    assert!(places.windows(2).all(|w| w[0] < w[1]), "out of order in start.rs");
    assert!(at(START, "let pid = mk_foreign_spawn(") < places[3]);
    let proved = at(START_GUEST, "start_time::reached(super::start_time::PROVED);");
    let loaded = at(START_GUEST, "start_time::reached(super::start_time::LOADED);");
    assert!(at(START_GUEST, "match prove(path, bytes") < proved && proved < loaded);
    assert!(loaded < at(START_GUEST, "mk_foreign_start("));
    assert!(START_GUEST.contains("let run = !super::gtk_prep::helping();"), "GTK's tools are not the run");
    let ensure = &ASK[at(ASK, "pub fn ensure(")..];
    assert!(at(ensure, "let granted = ask(front, port);") < at(ensure, "start_time::GRANT"));
}
