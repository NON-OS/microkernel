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

//! The backdoor call itself, for the calls and hosts `backdoor_rule` allows.

use core::arch::asm;

use super::backdoor_rule::{
    answered, call_allowed, host_answers, BACKDOOR_MAGIC, BACKDOOR_PORT, CMD_GETVERSION,
};
use super::state::current;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackdoorError {
    /// This host has no backdoor, or may have a device on its port.
    NoHost,
    /// A command, or an argument, outside the allowed set.
    Refused,
}

/// EAX, EBX, ECX and EDX as the host left them.
pub fn backdoor(cmd: u32, arg: u32) -> Result<[u32; 4], BackdoorError> {
    if !host_answers(current()) {
        return Err(BackdoorError::NoHost);
    }
    if !call_allowed(cmd, arg) {
        return Err(BackdoorError::Refused);
    }
    let (eax, ebx, ecx, edx): (u32, u64, u32, u32);
    // SAFETY: an I/O read the hypervisor traps, on a host named above. RBX is
    // LLVM's, so the argument travels in another register and is swapped in
    // and out around the read; RSI and RDI are given up, which some
    // backdoor commands write.
    unsafe {
        asm!(
            "xchg {b}, rbx",
            "in eax, dx",
            "xchg {b}, rbx",
            b = inout(reg) arg as u64 => ebx,
            inout("eax") BACKDOOR_MAGIC => eax,
            inout("ecx") cmd => ecx,
            inout("edx") BACKDOOR_PORT as u32 => edx,
            out("rsi") _,
            out("rdi") _,
            options(nomem, nostack, preserves_flags),
        );
    }
    Ok([eax, ebx as u32, ecx, edx])
}

/// Whether this host's backdoor answers: VMware's always does, QEMU's only
/// with its vmport device, which a machine type or `vmport=off` can leave out.
pub fn backdoor_present() -> bool {
    matches!(backdoor(CMD_GETVERSION, 0), Ok(regs) if answered(regs))
}
