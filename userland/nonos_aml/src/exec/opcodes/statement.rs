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

//! Statements (section 20.2.5.3).

use super::{noop, run, OpRow};

const FLOW: &str = "control flow; every term counts against the step budget";
const SYNC: &str = "on a mutex or event this evaluation may touch";

pub const ROWS: &[OpRow] = &[
    run(0xA0, "If", FLOW),
    run(0xA1, "Else", FLOW),
    run(0xA2, "While", "loops until the step budget, then faults"),
    run(0xA4, "Return", FLOW),
    run(0xA5, "Break", FLOW),
    run(0x9F, "Continue", FLOW),
    noop(0xA3, "Noop", "does nothing, as specified"),
    noop(0xCC, "BreakPoint", "no debugger is attached: passed over"),
    run(0x86, "Notify", "queued; the capsule routes it once the evaluation ends"),
    run(0x5B21, "Stall", "honoured, never shortened; counted against the time budget"),
    run(0x5B22, "Sleep", "the host sleeps; counted against the time budget"),
    run(0x5B24, "Signal", SYNC),
    run(0x5B26, "Reset", SYNC),
    run(0x5B27, "Release", "only by the evaluation that holds the mutex"),
    run(0x5B2A, "Unload", "only a table Load or LoadTable gave; anything else faults"),
    run(0x5B32, "Fatal", "said to the host, then the evaluation stops"),
];
