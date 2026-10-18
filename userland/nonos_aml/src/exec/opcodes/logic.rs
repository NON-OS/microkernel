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

//! The logical operators, the waits, the timer and the loads (section
//! 20.2.5.4).

use super::{run, OpRow};

const LOGIC: &str = "a comparison, Ones or Zero";

pub const ROWS: &[OpRow] = &[
    run(0x90, "LAnd", LOGIC),
    run(0x91, "LOr", LOGIC),
    run(0x92, "LNot", LOGIC),
    run(0x93, "LEqual", LOGIC),
    run(0x94, "LGreater", LOGIC),
    run(0x95, "LLess", LOGIC),
    run(0x5B23, "Acquire", "its timeout counted against the time budget"),
    run(0x5B25, "Wait", "its timeout counted against the time budget"),
    run(0x5B33, "Timer", "the host's clock in 100 ns ticks"),
    run(0x5B1F, "LoadTable", "a firmware table, measured before it loads (F-93)"),
    run(0x5B20, "Load", "a table from a region or buffer, measured before it loads (F-93)"),
];
