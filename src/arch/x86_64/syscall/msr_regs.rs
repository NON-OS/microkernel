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
//! The model-specific registers and flag bits the SYSCALL setup writes,
//! beside `msr.rs` so each file stays in bounds.

pub const IA32_EFER: u32 = 0xC0000080;
pub const IA32_STAR: u32 = 0xC0000081;
pub const IA32_LSTAR: u32 = 0xC0000082;
pub const IA32_CSTAR: u32 = 0xC0000083;
pub const IA32_FMASK: u32 = 0xC0000084;
pub const IA32_FS_BASE: u32 = 0xC0000100;
pub const IA32_GS_BASE: u32 = 0xC0000101;
pub const IA32_KERNEL_GS_BASE: u32 = 0xC0000102;

pub const EFER_SCE: u64 = 1 << 0;
pub const EFER_LME: u64 = 1 << 8;
pub const EFER_LMA: u64 = 1 << 10;
pub const EFER_NXE: u64 = 1 << 11;

pub const RFLAGS_IF: u64 = 1 << 9;
pub const RFLAGS_TF: u64 = 1 << 8;
pub const RFLAGS_DF: u64 = 1 << 10;
pub const RFLAGS_AC: u64 = 1 << 18;
pub const RFLAGS_NT: u64 = 1 << 14;

/// The flags SYSCALL clears on entry. NT among them: POPF sets it at CPL 3,
/// SYSCALL kept it, and the kernel's own IRETQ back to a preempted process
/// then raised #GP(0) (IRET in IA-32e mode with NT set), which the kernel
/// took as its own fault and panicked on. Any capsule could halt the machine.
pub const SYSCALL_FMASK: u64 = RFLAGS_IF | RFLAGS_TF | RFLAGS_DF | RFLAGS_AC | RFLAGS_NT;
