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

//! A write to the console: where it goes, for whom.

use super::carry::carry;
use crate::linux::abi::errno;
use crate::linux::console::wipe;
use crate::linux::guest::Guest;

/// Cap on one transfer, matching the kernel's own peer-copy ceiling.
const MAX_IO: u64 = 1 << 20;

/// A write of the guest's buffer to its console, `stream` 1 or 2. The bytes
/// are the guest's and are never interpreted, only forwarded, and zeroed
/// after.
pub(in super::super) fn console(guest: &Guest, buf: u64, len: u64, stream: u8) -> u64 {
    if len == 0 {
        return errno::ok(0);
    }
    let Some(mut bytes) = guest.read(buf, len.min(MAX_IO) as usize) else {
        return errno::fail(errno::EFAULT);
    };
    let sent = carry(&bytes, stream);
    wipe(&mut bytes);
    sent
}
