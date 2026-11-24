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

//! The start line said, once a run: when its guest family ends, or when it
//! stops before a guest.

use core::sync::atomic::{AtomicBool, Ordering::Relaxed};

use super::line::line;
use super::{since, AT, BEGUN, MAPPED, STAGES};

static SAID: AtomicBool = AtomicBool::new(false);

/// `how` is one word: `done` or `stopped`.
pub fn say(how: &str) {
    let Some(up) = BEGUN.load(Relaxed).checked_sub(1) else { return };
    if SAID.swap(true, Relaxed) {
        return;
    }
    let mut at = [0u64; STAGES];
    at.iter_mut().zip(&AT).for_each(|(to, from)| *to = from.load(Relaxed));
    let end = since().map_or(0, |ms| ms - 1);
    let text = line(up, &at, MAPPED.load(Relaxed), end, how);
    let _ = nonos_libc::mk_debug(text.as_ptr(), text.len());
}
