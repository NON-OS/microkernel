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

//! Where an asked-for install stands, and the one line a failed one leaves:
//! which package and which check. The kernel holds that line in memory for
//! the listing only, and never logs it.

use crate::syscall::{call_raw, N_MK_APP_INSTALL_STATUS};

/// The longest line the kernel keeps.
pub const MK_APP_INSTALL_DETAIL_MAX: usize = 120;

/// Where an asked-for install of `listing` stands: 0 nothing asked, 1 queued,
/// 2 installing, 3 installed, 4 refused before it started, 5 removing, 6
/// removed, 16 plus the installer's reason code when it failed.
pub fn mk_app_install_status(listing: &[u8]) -> i64 {
    call_raw(N_MK_APP_INSTALL_STATUS, [listing.as_ptr() as u64, listing.len() as u64, 0, 0, 0, 0])
}

/// The line `listing`'s failed install left, into `out`: its length, 0 when
/// there is none, or a negative errno. `out` takes the longest line whole.
pub fn mk_app_install_detail(listing: &[u8], out: &mut [u8; MK_APP_INSTALL_DETAIL_MAX]) -> i64 {
    let (l, o) = (listing.as_ptr() as u64, out.as_mut_ptr() as u64);
    call_raw(N_MK_APP_INSTALL_STATUS, [l, listing.len() as u64, 1, o, out.len() as u64, 0])
}

/// For the installer only: leave `line` for the listing it is installing,
/// cut to the longest the kernel keeps. 0, or a negative errno.
pub fn mk_app_install_tell(line: &[u8]) -> i64 {
    let n = line.len().min(MK_APP_INSTALL_DETAIL_MAX) as u64;
    call_raw(N_MK_APP_INSTALL_STATUS, [0, 0, 2, line.as_ptr() as u64, n, 0])
}
