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

//! CPU percentage from tick deltas.

use crate::rail::derive::cpu_pct;

#[test]
fn cpu_percent_is_the_tick_share() {
    assert_eq!(cpu_pct(25, 100), 25);
    assert_eq!(cpu_pct(1, 3), 33);
    assert_eq!(cpu_pct(0, 100), 0);
    assert_eq!(cpu_pct(100, 100), 100);
}

#[test]
fn a_zero_total_delta_reads_as_idle() {
    assert_eq!(cpu_pct(0, 0), 0);
    assert_eq!(cpu_pct(9_000, 0), 0);
}

#[test]
fn a_run_delta_over_the_total_clamps() {
    assert_eq!(cpu_pct(400, 100), 100);
    assert_eq!(cpu_pct(u64::MAX, 1), 100);
}
