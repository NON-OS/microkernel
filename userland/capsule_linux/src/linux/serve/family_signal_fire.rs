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

//! The timers of a process that are due: ITIMER_REAL raises SIGALRM.

use crate::linux::guest::siginfo::{SigInfo, SI_KERNEL};
use crate::linux::guest::sigstate::SIGALRM;
use crate::linux::guest::Guest;

/// Every timer of `g` due by `now`, fired.
pub fn fire(g: &mut Guest, now: u64) {
    if let Some(mut t) = g.signals.real.filter(|t| t.due <= now) {
        let _ = g.signals.raise(0, SigInfo::from(SIGALRM, SI_KERNEL, 0));
        g.signals.real = t.rearm(now).then_some(t);
    }
}
