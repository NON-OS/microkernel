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

use crate::syscall::{call_raw, N_MK_SURFACE_SHARE};

/// The receiver may write its view (MkSurfaceShare flags).
pub const SURFACE_SHARE_WRITE: u64 = 1;

// Share `sid` with `target_pid`, a live process other than the caller's: it
// and the caller are the only ones that may attach it. Without
// SURFACE_SHARE_WRITE the receiver maps it read-only. A fifth receiver of
// one surface is -ENOMEM; the owner's release takes every right back.
#[no_mangle]
pub extern "C" fn mk_surface_share(sid: u64, target_pid: u32, flags: u64) -> i64 {
    call_raw(N_MK_SURFACE_SHARE, [sid, target_pid as u64, flags, 0, 0, 0])
}

// Shares with `receiver_pid`'s process alone, read-only; call once per
// receiver. Nobody else can attach the handle, however they come by it.
#[no_mangle]
pub extern "C" fn mk_surface_share_to(sid: u64, receiver_pid: u32) -> i64 {
    call_raw(N_MK_SURFACE_SHARE, [sid, receiver_pid as u64, 0, 0, 0, 0])
}
