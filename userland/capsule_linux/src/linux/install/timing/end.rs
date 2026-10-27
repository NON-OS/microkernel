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

//! An install timed from its start to the lines it ends with, from the
//! counts timing/mod.rs keeps.

use core::sync::atomic::Ordering::Relaxed;

use nonos_libc::mk_debug;

use super::line::{moved, spent};
use super::{now, BYTES, GETS, N, NET_MS, PLACE_MS, START, VERIFY_MS};

/// The whole install, timed: begun, run, and its lines said however it ends.
pub fn timed<T>(install: impl FnOnce() -> T) -> T {
    begin();
    let done = install();
    end();
    done
}

/// An install begins: every count from zero, in the index phase.
fn begin() {
    N.iter().for_each(|c| c.store(0, Relaxed));
    N[START].store(now(), Relaxed);
}

/// The install ended, well or not: its three lines.
fn end() {
    let n = |at: usize| N[at].load(Relaxed);
    for (at, stage) in ["index", "packages"].into_iter().enumerate() {
        say(&moved(stage, n(GETS + at), n(BYTES + at), n(NET_MS + at)));
    }
    say(&spent(n(VERIFY_MS), n(PLACE_MS), now().saturating_sub(n(START))));
}

fn say(line: &str) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
