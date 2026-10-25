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

//! A capsule's payload, copied in to a buffer zeroed when it drops (C8).

use crate::ipc::nonos_channel::Payload;
use crate::syscall::microkernel::errnos::ERRNO_FAULT;

/// The `len` bytes at `buf`, or ERRNO_FAULT.
pub(super) fn copy_in(buf: u64, len: usize) -> Result<Payload, i64> {
    if crate::usercopy::validate_user_read(buf, len).is_err() {
        return Err(ERRNO_FAULT);
    }
    let mut data = Payload::zeroed(len);
    crate::usercopy::copy_from_user(buf, &mut data).map_err(|_| ERRNO_FAULT)?;
    Ok(data)
}
