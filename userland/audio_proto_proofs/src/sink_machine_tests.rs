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

//! The server's choice of the fixed sink: one registered is taken
//! without a question, two are asked in order, and a machine where neither
//! plays keeps the first, whose status says why.

use crate::sink_choose::{choose, SINKS, SINK_LINES};

#[test]
fn the_sinks_are_hda_then_ac97() {
    assert_eq!(SINKS, ["driver.hda0", "driver.ac97_0"]);
}

// The boot line names the sink chosen.
#[test]
fn each_sink_s_line_names_it() {
    for (sink, line) in SINKS.iter().zip(SINK_LINES) {
        assert_eq!(line, format!("[AUDIO] output {sink}\n"));
    }
}

#[test]
fn a_lone_sink_is_taken_without_asking() {
    let never = |_: usize| -> bool { panic!("a lone sink is not asked") };
    assert_eq!(choose([true, false], never), Some(0));
    assert_eq!(choose([false, true], never), Some(1));
}

#[test]
fn no_sink_is_none() {
    assert_eq!(choose([false, false], |_| true), None);
}

// An HD Audio controller that cannot play (HDMI only, a laptop for SOF)
// must not hide a working AC'97 beside it.
#[test]
fn with_both_the_first_that_plays_is_taken() {
    assert_eq!(choose([true, true], |i| i == 1), Some(1));
    let mut asked = Vec::new();
    assert_eq!(
        choose([true, true], |i| {
            asked.push(i);
            true
        }),
        Some(0)
    );
    assert_eq!(asked, vec![0], "the second is not asked once the first plays");
}

#[test]
fn with_both_and_neither_playing_the_first_keeps_its_say() {
    assert_eq!(choose([true, true], |_| false), Some(0));
}
