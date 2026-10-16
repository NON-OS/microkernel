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

//! The console's account of a stream: its first marks and its totals.

use alloc::format;

use nonos_libc::mk_debug;

use super::refill::Refill;

/*
 * The serial console hears about a stream three times at most: the first
 * period it played, the first underrun, and the totals when it stops. Every
 * other period is only counted, so a long playback or an idle stream cannot
 * fill the log.
 */
pub(super) const REFILL_MARK: &str = "[HDA] refill\n";
pub(super) const UNDERRUN_MARK: &str =
    "[HDA] underrun: the queue ran dry; counted, totals at stream stop\n";

impl Refill {
    /// One line with this run's totals, if it played anything, then a fresh
    /// count for the next run.
    pub(super) fn report(&mut self) {
        let p = self.periods;
        if p.any() {
            let line = format!(
                "[HDA] stream stop: {} periods played, {} underruns\n",
                p.played, p.underruns
            );
            mk_debug(line.as_ptr(), line.len());
        }
        self.reset();
    }
}

pub(super) fn mark(s: &str) {
    mk_debug(s.as_ptr(), s.len());
}
