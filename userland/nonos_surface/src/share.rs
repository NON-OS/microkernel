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

use nonos_abi::{syscall, N_SURFACE_SHARE};

/// The receiver may write its view.
pub const SHARE_WRITE: u64 = 1;

/// Share `sid` with `target_pid`, the one process besides the caller's that
/// may attach it; read-only unless `flags` holds SHARE_WRITE.
pub fn share(sid: u64, target_pid: u32, flags: u64) -> i64 {
    syscall(N_SURFACE_SHARE, [sid, target_pid as u64, flags, 0, 0, 0])
}
