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

//! Why the kernel would not start Qwen, in words and by errno name.

use alloc::vec::Vec;

use crate::term::state::State;
use crate::term::util::format_u64;

/// errno, its name, and what it means for a Qwen run.
const REASONS: &[(i64, &[u8], &[u8])] = &[
    (-2, b"ENOENT", b"this system was built without the Linux personality that runs Qwen"),
    (-22, b"EINVAL", b"the kernel does not know this tier"),
    (-16, b"EBUSY", b"every place for a Qwen chat is taken; end or close a running one first"),
    (-28, b"ENOSPC", b"no room to start another program"),
    (-12, b"ENOMEM", b"not enough memory to load the model"),
    (-11, b"EAGAIN", b"the system is busy; try again"),
    (-1, b"EPERM", b"this terminal may not start it"),
    (-13, b"EACCES", b"this terminal may not start it"),
    (-38, b"ENOSYS", b"this kernel cannot start programs by name"),
];

/// "qwen <tier>: <why> (<errno>)" on screen, and a failed status. For a
/// window, `tier` is "window <tier>".
pub fn refused(state: &mut State, tier: &[u8], rc: i64) {
    let mut line = Vec::with_capacity(96);
    line.extend_from_slice(b"qwen ");
    line.extend_from_slice(tier);
    line.extend_from_slice(b": ");
    match REASONS.iter().find(|(errno, _, _)| *errno == rc) {
        Some((_, name, why)) => {
            line.extend_from_slice(why);
            line.extend_from_slice(b" (");
            line.extend_from_slice(name);
        }
        None => {
            let mut num = [0u8; 20];
            let n = format_u64(rc.unsigned_abs(), &mut num);
            line.extend_from_slice(b"the kernel refused to start it (errno ");
            if rc < 0 {
                line.push(b'-');
            }
            line.extend_from_slice(&num[..n]);
        }
    }
    line.push(b')');
    state.scrollback.push_error(&line);
    state.last_status = 1;
}
