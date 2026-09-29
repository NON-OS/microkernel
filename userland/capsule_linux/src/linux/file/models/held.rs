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
 * Whether this family holds a model.
 *
 * A personality hosts one family, so this is the family's mark. It is set
 * when a model is first opened and never cleared: from then on the
 * family's console output reaches only its launcher, never the serial
 * log, and the family has no network, since nothing a model is shown or
 * says may leave the machine or land in a log.
 */

use core::sync::atomic::{AtomicBool, Ordering};

static HELD: AtomicBool = AtomicBool::new(false);

pub fn held() -> bool {
    HELD.load(Ordering::SeqCst)
}

/* Mark the family; said once, when the mark is first made. */
pub(super) fn hold() {
    if !HELD.swap(true, Ordering::SeqCst) {
        let line: &[u8] =
            b"[LINUX] family holds a model: console private, no network (net.sockets refused)\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
}
