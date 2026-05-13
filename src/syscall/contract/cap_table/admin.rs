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

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

// AdminReboot / AdminShutdown / AdminModLoad are reserved; the
// dispatcher returns ENOSYS until the corresponding admin capsule
// handlers land. AdminCapGrant / AdminCapRevoke have been removed —
// MkCapGrant / MkCapRevoke are the single source of truth for
// capability operations.
pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        SyscallNumber::AdminReboot
        | SyscallNumber::AdminShutdown
        | SyscallNumber::AdminModLoad => caps.can_admin(),

        _ => return None,
    })
}
