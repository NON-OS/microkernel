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

//! The serve loop's pass, as the streams it carries see it: at most one
//! quiet stream waits on net.socks5 in a pass (quiet.rs), and none once the
//! pass has delivered input to a window, so what the input changed is not
//! held behind a poll that brings nothing.

use core::sync::atomic::{AtomicBool, Ordering};

/// Whether this pass's one quiet wait is spent.
static SPENT: AtomicBool = AtomicBool::new(false);

/// A new pass of the serve loop begins.
pub fn begin() {
    SPENT.store(false, Ordering::Relaxed);
}

/// Take the pass's quiet wait: false once it is spent.
pub fn take() -> bool {
    !SPENT.swap(true, Ordering::Relaxed)
}

/// Input went to a window in this pass: no quiet stream waits in it.
pub fn spend() {
    SPENT.store(true, Ordering::Relaxed);
}
