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

/*
 * What the view says of one process: its image, whether it waits,
 * and its dispositions.
 */

use crate::linux::file::{self};
use crate::linux::guest::sigstate::NSIG;
use crate::linux::guest::Guest;

/* A forked child runs its parent's image until it execs. */
pub(super) fn image_of(all: &[Guest], g: &Guest) -> file::Exe {
    let mut at = g.pid;
    for _ in 0..all.len() + 1 {
        if let Some(exe) = file::exe_of(at) {
            return exe;
        }
        match all.iter().find(|p| p.children.contains(&at)) {
            Some(p) => at = p.pid,
            None => break,
        }
    }
    file::Exe::default()
}

/* Some thread of it is parked in a call. */
pub(super) fn parked(g: &Guest) -> bool {
    let mut tids = core::iter::once(g.pid).chain(g.threads.iter().copied());
    tids.any(|t| g.parked(t).is_some()) || g.signals.vfork.is_some()
}

/* The signals it catches and the ones it ignores, as status's masks. */
pub(super) fn dispositions(g: &Guest) -> (u64, u64) {
    let (mut caught, mut ignored) = (0u64, 0u64);
    for n in 1..=NSIG {
        match g.signals.action(n) {
            Some(a) if a.catches() => caught |= 1 << (n - 1),
            Some(a) if a.ignores() => ignored |= 1 << (n - 1),
            _ => {}
        }
    }
    (caught, ignored)
}
