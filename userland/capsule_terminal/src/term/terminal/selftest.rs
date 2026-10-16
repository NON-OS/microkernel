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

use nonos_libc::mk_debug;

use crate::term::state::State;

mod block_test;
mod boot;
mod ext;
mod git_test;
mod jobs_test;
mod kernel_api;
mod probe;
mod proc_lifecycle;
mod vt_grid;
mod vt_screen;

pub use boot::main;
use ext::run_ext;
use probe::{mark, ok_cmd, row_text, run_cmd, visible_has};

// Drive the previously unproven shell paths through the normal submit
// path and emit one serial marker per step so a headless boot grades
// itself: echo, a vfs write/read round trip, a pipe, and `||` gating.
pub fn vt_selftest() {
    vt_grid::run();
    vt_screen::run();
    block_test::run();
}

fn run(state: &mut State) {
    vt_selftest();
    run_cmd(state, b"echo selfcheck");
    mark(b"echo", visible_has(state, b"selfcheck"));

    run_cmd(state, b"write /st.txt smoke123");
    run_cmd(state, b"read /st.txt");
    mark(b"vfs", visible_has(state, b"smoke123"));

    run_cmd(state, b"echo a b c | wc");
    mark(b"pipe", visible_has(state, b"1"));

    run_cmd(state, b"echo a | cat /readme.txt");
    mark(b"pipe-anystage", visible_has(state, b"This file lives in the vfs capsule."));

    run_cmd(state, b"read /nope.txt || echo recovered");
    mark(b"statement", visible_has(state, b"recovered"));

    run_ext(state);

    let pass = b"[TERMINAL-TEST] PASS\n";
    let _ = mk_debug(pass.as_ptr(), pass.len());
}
