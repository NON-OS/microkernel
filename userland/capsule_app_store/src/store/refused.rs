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

//! What the reader is told when the system refuses an install, an
//! uninstall or a start outright.
//!
//! Every refusal but EBUSY read "the system refused the request", which
//! gave no way to tell a listing the system does not take from a window
//! that may not ask. The kernel's answers (src/syscall/microkernel/
//! app_install.rs, app_uninstall.rs, app_launch.rs, and the AppInstall
//! gate in front of them) each get their own words.
//!
//! Pure; market_proofs holds it.

const EPERM: i64 = -1;
const EFAULT: i64 = -14;
const EACCES: i64 = -13;
const EINVAL: i64 = -22;

/// The status line's words for a request the system refused with `errno`.
pub fn refused(errno: i64) -> &'static [u8] {
    match errno {
        EINVAL => b"the system does not take this listing: it is not a package it installs",
        EPERM | EACCES => b"this window is not allowed to ask that (it lacks AppInstall)",
        EFAULT => b"the system could not read the request; ask again",
        _ => b"the system refused the request; the serial log says why",
    }
}
