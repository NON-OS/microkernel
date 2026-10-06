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

use core::sync::atomic::{AtomicUsize, Ordering};

/* Compound tests spent by matches_selector, counted across the whole run.
 * The count only grows (wrapping), so a cascade reads its own share as the
 * difference from where it started and no reset is needed. */
static SPENT: AtomicUsize = AtomicUsize::new(0);

pub(super) fn charge(n: u32) {
    SPENT.fetch_add(n as usize, Ordering::Relaxed);
}

pub fn spent() -> usize {
    SPENT.load(Ordering::Relaxed)
}
