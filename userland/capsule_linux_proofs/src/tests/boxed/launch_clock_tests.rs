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

//! The line a launch says at its first window: each count by its words,
//! the bytes read in KiB, said once, from the first window opened.

#[path = "../../../../capsule_linux/src/linux/start_say/clock_line.rs"]
mod clock_line;

use clock_line::{line, Counts};

const WINDOW: &str = include_str!("../../../../capsule_linux/src/linux/wayland/present/window.rs");
const CLOCK: &str = include_str!("../../../../capsule_linux/src/linux/start_say/clock.rs");

#[test]
fn the_line_names_each_count() {
    let c = Counts {
        ms: 2950,
        prep_ms: 1200,
        helpers: 6,
        reads: 840,
        read_bytes: 5 << 20,
        calls: 41000,
    };
    let want = "[LINUX-RUN] window after 2950 ms: GTK preparation 1200 ms in 6 runs, 840 store \
                reads (5120 KiB), 41000 calls\n";
    assert_eq!(line(&c), want);
    let small = Counts { read_bytes: 1023, ..c };
    assert!(line(&small).contains("(0 KiB)"), "a part of a KiB rounds down");
}

#[test]
fn it_is_said_once_from_the_first_window_opened() {
    let at = |needle: &str| WINDOW.find(needle).unwrap_or_else(|| panic!("{needle} is gone"));
    assert!(at("let opened = opened.is_some();") < at("if opened {"));
    assert!(at("if opened {") < at("clock::window();"));
    assert!(CLOCK.contains("if SAID.swap(true, Relaxed) {\n        return;\n    }"));
}
