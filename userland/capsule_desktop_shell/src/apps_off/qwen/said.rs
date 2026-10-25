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

/* What the last open said: the default it ran, its answer, and why not. */

use alloc::vec::Vec;
use core::sync::atomic::Ordering;

use super::default_tier::WEIGHTS;
use super::run::{DEFAULT, LAST};

/*
 * Said once a window opened on a default: "Qwen: none chosen, default
 * qwen3-0.6b", short enough for a toast. None for the person's choice.
 */
pub fn default_said() -> Option<Vec<u8>> {
    let at = usize::from(DEFAULT.swap(0, Ordering::Relaxed)).checked_sub(1)?;
    Some([&b"Qwen: none chosen, default "[..], WEIGHTS[at].0.as_bytes()].concat())
}

/* The kernel's answer to the last open, for the log line beside `why`. */
pub fn last() -> i64 {
    LAST.load(Ordering::Relaxed)
}

/* Why the last click opened nothing, as the Terminal's qwen says it. A tier
 * with no model is not a refusal: the window opens and says so itself. */
pub fn why() -> &'static [u8] {
    match LAST.load(Ordering::Relaxed) {
        -2 => b"Qwen is not in this system",
        -22 => b"Qwen: the kernel does not know this tier",
        -16 => b"Qwen: every chat place is taken; close one",
        -28 => b"Qwen: no room to start another program",
        -12 => b"Qwen: not enough memory for this model",
        -11 => b"Qwen: the system is busy; try again",
        -13 => b"Linux and Qwen turned off at setup",
        _ => b"Qwen could not start; qwen in Terminal says why",
    }
}
