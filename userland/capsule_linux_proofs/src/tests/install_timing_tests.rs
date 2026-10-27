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

//! An install's [LINUX-TIME] lines (install/timing/line.rs): sizes in MB
//! to one decimal, seconds to one decimal, MB/s to two, and no package name.

#[path = "../../../capsule_linux/src/linux/install/timing/line.rs"]
mod line;

#[test]
fn a_stage_says_its_gets_bytes_time_and_rate() {
    let monero = line::moved("packages", 25, 32_500_000, 98_000);
    assert_eq!(monero, "[LINUX-TIME] packages 25 gets 32.5 MB 98.0 s 0.33 MB/s\n");
    let index = line::moved("index", 2, 2_322_000, 4_150);
    assert_eq!(index, "[LINUX-TIME] index 2 gets 2.3 MB 4.1 s 0.55 MB/s\n");
}

#[test]
fn nothing_timed_has_no_rate() {
    assert_eq!(line::moved("index", 0, 0, 0), "[LINUX-TIME] index 0 gets 0.0 MB 0.0 s - MB/s\n");
}

#[test]
fn the_rest_is_verify_place_and_total() {
    let line = line::spent(3_250, 5_049, 112_300);
    assert_eq!(line, "[LINUX-TIME] verify 3.2 s place 5.0 s total 112.3 s\n");
}
