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

use crate::syscall::numbers::SyscallNumber;
use crate::syscall::SyscallResult;

use super::{admin, crypto, graphics_backend, input_ops, microkernel_ops, surface_ops};

pub(super) fn dispatch_syscall(
    syscall: SyscallNumber,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> SyscallResult {
    /*
     * Hot syscalls first: a frame, input and IPC loop makes these many times
     * a frame. Each arm routes where the general match below routes it; the
     * order only spares them the crypto, admin and microkernel-ops tests
     * (microkernel_ops::matches alone names some 160 numbers).
     */
    match syscall {
        SyscallNumber::MkSurfacePresent | SyscallNumber::MkDisplayVsyncWait => {
            return surface_ops::handle(syscall, a0, a1, a2, a3, a4, a5);
        }
        SyscallNumber::MkInputEventDrain | SyscallNumber::MkInputEventWait => {
            return input_ops::handle(syscall, a0, a1, a2, a3, a4, a5);
        }
        SyscallNumber::MkIpcSend
        | SyscallNumber::MkIpcRecv
        | SyscallNumber::MkIpcCall
        | SyscallNumber::MkIpcRecvFrom
        | SyscallNumber::MkIpcReply
        | SyscallNumber::MkIpcRecvReq
        | SyscallNumber::MkIpcReplyTo
        | SyscallNumber::MkIpcSendToPid
        | SyscallNumber::MkFutexWait
        | SyscallNumber::MkFutexWake => {
            return microkernel_ops::handle(syscall, a0, a1, a2, a3, a4, a5);
        }
        _ => {}
    }
    match syscall {
        SyscallNumber::CryptoRandom
        | SyscallNumber::CryptoHash
        | SyscallNumber::CryptoEncrypt
        | SyscallNumber::CryptoDecrypt
        | SyscallNumber::CryptoEncryptAad
        | SyscallNumber::CryptoDecryptAad
        | SyscallNumber::CryptoX25519Public
        | SyscallNumber::CryptoX25519Shared
        | SyscallNumber::CryptoHmacSha256
        | SyscallNumber::CryptoHkdfSha256
        | SyscallNumber::CryptoKeccak256
        | SyscallNumber::CryptoMachineKey => {
            crypto::dispatch_crypto(syscall, a0, a1, a2, a3, a4, a5)
        }
        nr if admin::matches(nr) => admin::handle(nr, a0, a1, a2, a3, a4, a5),
        nr if microkernel_ops::matches(nr) => microkernel_ops::handle(nr, a0, a1, a2, a3, a4, a5),
        nr if graphics_backend::matches(nr) => graphics_backend::handle(nr, a0, a1, a2, a3, a4, a5),
        SyscallNumber::MkSurfaceRegister
        | SyscallNumber::MkSurfaceShare
        | SyscallNumber::MkSurfaceAttach
        | SyscallNumber::MkSurfaceRelease
        | SyscallNumber::MkSurfacePresent
        | SyscallNumber::MkDisplayVsyncWait
        | SyscallNumber::MkSurfaceFenceRelease
        | SyscallNumber::MkSurfaceFenceWait => surface_ops::handle(syscall, a0, a1, a2, a3, a4, a5),
        SyscallNumber::MkInputEventPost
        | SyscallNumber::MkInputEventDrain
        | SyscallNumber::MkInputEventWait => input_ops::handle(syscall, a0, a1, a2, a3, a4, a5),
        _ => crate::syscall::dispatch::util::errno(38),
    }
}
