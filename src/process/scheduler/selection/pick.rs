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

//! Which runnable process to try next: the highest priority band that has a
//! Ready one, round robin by pid within it.

use super::super::dispatch::get_runnable_pids;
use super::band_choice::choose;
use super::band_scan::{candidates, select_fallback};
use super::select::LAST_PER_BAND;
use core::sync::atomic::Ordering;

/// The candidate and, if it came from a priority band, that band's index. The
/// cursors are not advanced here: a pick that loses its claim never ran.
pub(super) fn pick() -> Option<(u32, Option<usize>)> {
    use crate::process::nonos_core::CURRENT_PID;
    let current = CURRENT_PID.load(Ordering::Relaxed);
    let mut runnable = get_runnable_pids();
    if runnable.is_empty() {
        return None;
    }
    // Sorted for the table's one pass; the order chosen in does not depend on
    // the queue's order, only on pids and the bands' last picks.
    runnable.sort_unstable();
    let last = core::array::from_fn(|band| LAST_PER_BAND[band].load(Ordering::Relaxed));
    if let Some((pid, band)) = choose(&candidates(&runnable, current), &last) {
        return Some((pid, Some(band)));
    }
    select_fallback(&runnable, current).map(|pid| (pid, None))
}
