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

//! The pure half of `linux --box`, as the capsule builds it: the presets and
//! their hash, the refusal kinds and tally, and the report. The armed state
//! that holds them in the capsule (state.rs) reaches the serve loop, not here.

#[path = "../../../capsule_linux/src/linux/boxed/allowed.rs"]
pub mod allowed;
#[path = "../../../capsule_linux/src/linux/boxed/connected.rs"]
pub mod connected;
#[path = "../../../capsule_linux/src/linux/boxed/dests.rs"]
pub mod dests;
#[path = "../../../capsule_linux/src/linux/boxed/end.rs"]
pub mod end;
#[path = "../../../capsule_linux/src/linux/boxed/folders.rs"]
pub mod folders;
#[path = "../../../capsule_linux/src/linux/boxed/frames.rs"]
pub mod frames;
#[path = "../../../capsule_linux/src/linux/boxed/hash.rs"]
pub mod hash;
#[path = "../../../capsule_linux/src/linux/boxed/items.rs"]
pub mod items;
#[path = "../../../capsule_linux/src/linux/boxed/kind.rs"]
pub mod kind;
#[path = "../../../capsule_linux/src/linux/boxed/learn_folders.rs"]
pub mod learn_folders;
#[path = "../../../capsule_linux/src/linux/boxed/learn_report.rs"]
pub mod learn_report;
#[path = "../../../capsule_linux/src/linux/boxed/learned.rs"]
pub mod learned;
#[path = "../../../capsule_linux/src/linux/boxed/learned_text.rs"]
pub mod learned_text;
#[path = "../../../capsule_linux/src/linux/boxed/line.rs"]
pub mod line;
#[path = "../../../capsule_linux/src/linux/boxed/net_report.rs"]
pub mod net_report;
#[path = "../../../capsule_linux/src/linux/boxed/preset.rs"]
pub mod preset;
#[path = "../../../capsule_linux/src/linux/boxed/report.rs"]
pub mod report;
#[path = "../../../capsule_linux/src/linux/boxed/run_line.rs"]
pub mod run_line;
#[path = "../../../capsule_linux/src/linux/boxed/tally.rs"]
pub mod tally;
#[path = "../../../capsule_linux/src/linux/boxed/terminal_run.rs"]
pub mod terminal_run;

/* The selftest image's deliberate panic, off unless the feature is on. */
#[path = "../../../capsule_linux/src/linux/selftest_panic.rs"]
pub mod selftest_panic;

/* Whether a family's app is noted to the seal's slot 0 (I-C6). */
#[path = "../../../capsule_linux/src/linux/seal_slot.rs"]
pub mod seal_slot;

/* What every word `--box` takes must leave, for the fuzz target and the sweep. */
#[cfg(any(test, fuzzing))]
pub mod check;
