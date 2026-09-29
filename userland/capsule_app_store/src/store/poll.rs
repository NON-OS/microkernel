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

//! Asking the system where each install stands, while any is moving.

use nonos_libc::mk_app_install_status;

use super::progress::Progress;
use super::state::State;

impl State {
    /// Ask about every Linux listing once, so one installed earlier in the
    /// session reads as installed. True when anything changed.
    pub fn poll_all(&mut self) -> bool {
        self.poll(|l| l.id.starts_with(b"linux."))
    }

    /// Ask about the installs still moving. True when anything changed.
    pub fn poll_pending(&mut self) -> bool {
        self.poll(|l| l.progress.pending())
    }

    pub fn any_pending(&self) -> bool {
        self.listings.iter().any(|l| l.progress.pending())
    }

    fn poll(&mut self, which: impl Fn(&super::listing::Listing) -> bool) -> bool {
        let mut changed = false;
        for l in self.listings.iter_mut().filter(|l| which(l)) {
            let now = Progress::of(mk_app_install_status(&l.id));
            // A request this window made is not forgotten by an early answer.
            let now = if now == Progress::Idle && l.progress.pending() { l.progress } else { now };
            changed |= now != l.progress;
            l.progress = now;
        }
        changed
    }
}
