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

//! Process and IPC calls, and granting capabilities.

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        SyscallNumber::MkGetPid => caps.can_getpid(),
        SyscallNumber::MkArgs => caps.can_getpid(),
        SyscallNumber::MkThreadSpawn => caps.can_ipc(),
        /*
         * A capsule may only move its own fs base; that mutates nothing
         * outside its own PCB, so a valid token is the whole requirement.
         */
        SyscallNumber::MkSetTls => caps.is_valid(),
        SyscallNumber::MkProcOutput => caps.can_ipc(),
        SyscallNumber::MkProcInput => caps.can_ipc(),
        SyscallNumber::MkStdinRead => caps.can_ipc(),
        SyscallNumber::MkWait => caps.can_ipc(),
        SyscallNumber::MkKill => caps.can_ipc(),

        SyscallNumber::MkIpcCall
        | SyscallNumber::MkIpcRecv
        | SyscallNumber::MkIpcRecvFrom
        | SyscallNumber::MkIpcReply
        | SyscallNumber::MkIpcSend
        | SyscallNumber::MkIpcSendToPid
        | SyscallNumber::MkServiceLookup
        | SyscallNumber::MkThreadGroup
        | SyscallNumber::MkServiceRegister => caps.can_ipc(),
        /*
         * The handlers ask for Admin again and refuse a bit the caller lacks;
         * the table asks first, so a capsule without Admin never reaches them.
         */
        SyscallNumber::MkCapGrant | SyscallNumber::MkCapRevoke => caps.can_admin(),

        _ => return None,
    })
}
