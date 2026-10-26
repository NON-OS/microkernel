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

use crate::sys::serial;

#[cfg(feature = "nonos-user-entry-proof")]
pub(super) fn print_syscall_msrs() {
    use crate::arch::x86_64::syscall::msr::{
        read_msr, EFER_SCE, IA32_EFER, IA32_FMASK, IA32_LSTAR, IA32_STAR,
    };
    let line = |name: &[u8], v: u64| serial::Line::new().str(name).hex(v).end();
    let efer = read_msr(IA32_EFER);
    let mut l = serial::Line::new();
    l.str(b"[SYSCALL-MSR] EFER=").hex(efer).str(b" SCE=").hex(efer & EFER_SCE).end();
    line(b"[SYSCALL-MSR] STAR=", read_msr(IA32_STAR));
    line(b"[SYSCALL-MSR] LSTAR=", read_msr(IA32_LSTAR));
    line(b"[SYSCALL-MSR] SFMASK=", read_msr(IA32_FMASK));
}
