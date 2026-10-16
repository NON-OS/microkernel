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

//! Which refusals have been told: each distinct (space, base, length) once,
//! the first `BOUND` of them, then one line saying the rest go untold, so
//! AML retrying or walking a refused range cannot fill the log. Every
//! refusal is still counted in the posture line.

use super::super::rule::{tell, Tell};
use crate::sys::serial::Line;
use alloc::vec::Vec;
use spin::Mutex;

const BOUND: usize = 64;

static TOLD: Mutex<Vec<(u8, u64, u64)>> = Mutex::new(Vec::new());

pub(super) fn told(space: u8, base: u64, len: u64) -> Tell {
    let key = (space, base, len);
    let mut told = TOLD.lock();
    let verdict = tell(told.contains(&key), told.len(), BOUND);
    if verdict != Tell::Quiet && told.try_reserve(1).is_ok() {
        told.push(key);
    }
    verdict
}

pub(super) fn say(verdict: Tell, mut line: Line, why: &[u8]) {
    line.str(b" refused: ").str(why);
    if verdict == Tell::Last {
        line.str(b"; further refusals are counted, not told");
    }
    line.end();
}
