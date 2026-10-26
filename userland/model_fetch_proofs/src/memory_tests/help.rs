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

//! What `help qwen` says a tier needs.

/* The Terminal's `help qwen`, as it ships. */
#[path = "../../../capsule_terminal/src/command/builtin/qwen/help.rs"]
mod qwen_help;

/*
 * What `help qwen` says a tier needs beyond its file is what need.rs, the
 * rule the fetcher and the Store hold a download to, adds for the tier that
 * needs most: never a figure larger or smaller than the one in force.
 */
#[test]
fn help_says_what_a_tier_needs_beyond_its_file() {
    let pinned = |tier: &str| -> u64 {
        crate::pins::all().filter(|p| p.tier == tier).map(|p| p.bytes).sum()
    };
    let beyond = |s: &(&str, u64, u64, u64, u64)| {
        crate::need::memory(s.0, pinned(s.0)).unwrap() - pinned(s.0)
    };
    let most = crate::need::SHAPES.iter().map(beyond).max().unwrap();
    assert!(most <= 910_000_000 && most > 900_000_000, "the most beyond a file is {most}");
    let help: Vec<String> =
        qwen_help::HELP.iter().map(|l| String::from_utf8_lossy(l).into_owned()).collect();
    assert!(help.iter().any(|l| l.contains("plus up to 0.91 GB")), "{help:?}");
    assert!(!help.iter().any(|l| l.contains("1-2 GB")), "{help:?}");
}
