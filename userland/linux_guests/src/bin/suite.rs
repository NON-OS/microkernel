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

//! Every single-process probe, in one run.
//!
//! One boot then carries the evidence for all of them. Each probe still
//! reports under its own name, so a refusal in the log says which property it
//! belongs to.

use std::process::ExitCode;

use nonos_linux_guests::report::Report;
use nonos_linux_guests::{
    bounds_probe, exec_probe, fs_probe, life_probe, native_probe, proc_probe, sep_probe,
};

fn main() -> ExitCode {
    let mut broken = false;
    for (guest, scan) in [
        ("native", native_probe::scan as fn(&mut Report)),
        ("bounds", bounds_probe::scan),
        ("fs", fs_probe::scan),
        ("proc", proc_self),
        ("separation", sep_probe::scan),
        ("exec", exec_probe::scan),
    ] {
        let mut r = Report::new(guest);
        scan(&mut r);
        broken |= r.finish() != ExitCode::SUCCESS;
    }
    // Last, since a personality that loses the child may lose this process.
    life_probe::run();
    ExitCode::from(u8::from(broken))
}

/// /proc of other pids, without a sibling: nothing should open at all.
fn proc_self(r: &mut Report) {
    let pids: Vec<u32> = (1..=256).collect();
    proc_probe::scan(r, &pids);
}
