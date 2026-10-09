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

//! Reading a sibling through /proc, the way a debugger would.

use crate::report::{Report, Seen};
use crate::sys::{call, CLOSE, OPEN, READ};

const O_RDONLY: u64 = 0;

/// /proc/<pid>/mem and /proc/<pid>/maps. Opening either for another process
/// is already too much, whatever a read would then return.
pub fn scan(r: &mut Report, pids: &[u32]) {
    for leaf in ["mem", "maps"] {
        let mut seen = Seen::Refused(-1);
        for &pid in pids {
            let path = format!("/proc/{pid}/{leaf}\0");
            let fd = call(OPEN, [path.as_ptr() as u64, O_RDONLY, 0, 0, 0, 0]);
            if fd >= 0 {
                let mut buf = [0u8; 64];
                let n = call(READ, [fd as u64, buf.as_mut_ptr() as u64, 64, 0, 0, 0]);
                let _ = call(CLOSE, [fd as u64, 0, 0, 0, 0, 0]);
                seen = Seen::Escaped(format!("opened /proc/{pid}/{leaf}, read {n}"));
                break;
            }
            seen = Seen::Refused(fd);
        }
        r.check(&format!("/proc/pid/{leaf}"), seen);
    }
}
