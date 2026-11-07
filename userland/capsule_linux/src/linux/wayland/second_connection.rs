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

//! A second display connection refused (window_life/ends.rs may_connect),
//! said once a run: a client that retries would fill the Terminal.

use core::sync::atomic::{AtomicBool, Ordering};

static SAID: AtomicBool = AtomicBool::new(false);

pub fn say_second_connection() {
    if !SAID.swap(true, Ordering::Relaxed) {
        let line = b"[WAYLAND] a second display connection refused while one is open\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
}
