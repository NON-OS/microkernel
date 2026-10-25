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

//! A grant taken back out of its capsule's domain.

use super::iova;
use super::table::CAPSULES;

/// True once no device can reach the grant, which is when its frames may be
/// scrubbed and handed to someone else. False means they must not be.
pub(in crate::hardware::broker) fn unmap(pid: u32, iova: u64, length: u64, confined: bool) -> bool {
    if !confined {
        return true;
    }
    let mut all = CAPSULES.lock();
    let Some(c) = all.iter_mut().find(|c| c.pid == pid) else {
        // The capsule's last device was detached, which denied it this too.
        return true;
    };
    if c.domain.unmap(iova, length as usize).is_err() {
        return false;
    }
    iova::give_back(c, iova, length);
    true
}
