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

//! Says on the log which of the lease's DNS servers answered, the first time
//! and whenever another one does, so a photo shows DNS working through the
//! router without a line for every lookup. The number is the server's place
//! among the "lease dns" lines.

use core::sync::atomic::{AtomicUsize, Ordering};

use nonos_libc::mk_debug;

static LAST: AtomicUsize = AtomicUsize::new(usize::MAX);

pub(super) fn say_answered(server: usize) {
    if server > 8 || LAST.swap(server, Ordering::Relaxed) == server {
        return;
    }
    let mut line = *b"[NET-CORE] dns: answered by lease server 0\n";
    let digit = line.len() - 2;
    line[digit] = b'1' + server as u8;
    mk_debug(line.as_ptr(), line.len());
}
