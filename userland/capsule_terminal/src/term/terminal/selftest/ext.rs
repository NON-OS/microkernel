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

//! The coreutils pass graded: path tools, cut, redirection, find, clock, net.

use super::{mark, ok_cmd, run_cmd, visible_has};
use crate::term::state::State;

// Grade the commands added for the Warp/coreutils pass: path utilities,
// the cut filter, touch+&&, recursive find, and clock/size queries that
// only assert success because their output is not deterministic.
pub(super) fn run_ext(state: &mut State) {
    run_cmd(state, b"basename /a/b/file.txt");
    mark(b"basename", visible_has(state, b"file.txt"));

    run_cmd(state, b"dirname /a/b/file.txt");
    mark(b"dirname", visible_has(state, b"/a/b"));

    run_cmd(state, b"echo a:b:c | cut -d: -f2");
    mark(b"cut", visible_has(state, b"b"));

    run_cmd(state, b"write /rin.txt redirin");
    run_cmd(state, b"grep redirin < /rin.txt");
    mark(b"redir-in", visible_has(state, b"redirin"));

    run_cmd(state, b"touch /tt.txt && echo created");
    mark(b"touch", visible_has(state, b"created"));

    run_cmd(state, b"find /");
    mark(b"find", visible_has(state, b"/readme.txt"));

    mark(b"du", ok_cmd(state, b"du /"));
    mark(b"date", ok_cmd(state, b"date"));

    run_cmd(state, b"ifconfig");
    mark(b"ifconfig", visible_has(state, b"net0: down"));

    run_cmd(state, b"nslookup nonos.test");
    mark(b"nslookup", visible_has(state, b"nslookup: dns unavailable"));

    run_cmd(state, b"cat /nonexistent");
    run_cmd(state, b"echo $?");
    mark(b"status", visible_has(state, b"1"));

    super::kernel_api::run();
    super::proc_lifecycle::run();
    super::jobs_test::run(state);
    super::git_test::run(state);
}
