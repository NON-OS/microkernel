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

use crate::process::nonos_context::CpuContext;
use alloc::boxed::Box;

pub use super::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ThreadState {
    Ready = 0,
    Running = 1,
    Blocked = 2,
    Sleeping = 3,
    Zombie = 4,
    Stopped = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockReason {
    Io,
    Lock,
    Futex(u64),
    Wait,
    Signal,
    Ipc,
}

#[repr(C, align(4096))]
pub struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: [0; KERNEL_STACK_SIZE] })
    }

    pub fn top(&self) -> u64 {
        let ptr = self.data.as_ptr() as u64;
        ptr + KERNEL_STACK_SIZE as u64
    }

    pub fn base(&self) -> u64 {
        self.data.as_ptr() as u64
    }
}

impl Default for KernelStack {
    fn default() -> Self {
        Self { data: [0; KERNEL_STACK_SIZE] }
    }
}

// The x86_64 area holds every XSAVE component the boot CPU enabled.
#[cfg(target_arch = "x86_64")]
const FPU_AREA: usize = crate::arch::x86_64::cpu::xstate::AREA;
#[cfg(not(target_arch = "x86_64"))]
const FPU_AREA: usize = 1024;

// Never on a stack: the area is 4 KiB and 64-byte aligned, and a kernel stack
// that held one per switch overflowed. `new` builds it on the heap, zeroed.
#[repr(C, align(64))]
pub struct FpuState {
    pub data: [u8; FPU_AREA],
}

impl FpuState {
    pub fn new() -> Box<Self> {
        // SAFETY: eK@nonos.systems - FpuState is plain bytes, so all zeros is a
        // valid value, and a zeroed area is also a clear XSAVE header.
        unsafe { Box::<Self>::new_zeroed().assume_init() }
    }

    #[inline(always)]
    pub fn save(&mut self) {
        // XSAVE over every component XCR0 enables when the boot CPU turned it
        // on, FXSAVE otherwise. The aarch64 register file is saved by
        // `arch::aarch64::fpu`, so this body is x86_64 only.
        #[cfg(target_arch = "x86_64")]
        // SAFETY: eK@nonos.systems - `data` is AREA bytes, 64-byte aligned by
        // repr(align(64)), and `record_boot` keeps the enabled components within
        // AREA. The area is zeroed when made, so the XSAVE header starts clear.
        unsafe {
            if crate::arch::x86_64::cpu::xstate::uses_xsave() {
                core::arch::asm!("xsave64 [{}]", in(reg) self.data.as_mut_ptr(),
                    in("eax") u32::MAX, in("edx") u32::MAX, options(nostack, preserves_flags));
            } else {
                core::arch::asm!("fxsave64 [{}]", in(reg) self.data.as_mut_ptr(),
                    options(nostack, preserves_flags));
            }
        }
    }

    #[inline(always)]
    pub fn restore(&self) {
        #[cfg(target_arch = "x86_64")]
        // SAFETY: eK@nonos.systems - reads back an area `save` wrote on a CPU
        // with the same XCR0, which `mirror_on_ap` guarantees for every CPU.
        unsafe {
            if crate::arch::x86_64::cpu::xstate::uses_xsave() {
                core::arch::asm!("xrstor64 [{}]", in(reg) self.data.as_ptr(),
                    in("eax") u32::MAX, in("edx") u32::MAX, options(nostack, preserves_flags));
            } else {
                core::arch::asm!("fxrstor64 [{}]", in(reg) self.data.as_ptr(),
                    options(nostack, preserves_flags));
            }
        }
    }

    /// A new thread's unit: every register zero, FCW 0x037F, MXCSR 0x1F80.
    /// FNINIT and LDMXCSR alone left xmm and the ymm upper halves holding the
    /// last thread's values, which the state suite read from a sibling.
    #[inline(always)]
    pub fn init() {
        CLEAN.restore();
    }
}

/// The initial state, as an area to restore: control words set, registers
/// zero, and an XSAVE header whose empty XSTATE_BV puts every component the
/// area covers in its initial configuration. MXCSR is 0x1F80, every SIMD
/// exception masked: an all-zero area would unmask them all.
static CLEAN: FpuState = FpuState::clean();

impl FpuState {
    const fn clean() -> Self {
        let mut data = [0u8; FPU_AREA];
        data[0] = 0x7F;
        data[1] = 0x03;
        data[24] = 0x80;
        data[25] = 0x1F;
        Self { data }
    }
}

pub struct ThreadControlBlock {
    pub tid: u64,
    pub pid: u64,
    pub name: [u8; 32],
    pub state: ThreadState,
    pub context: CpuContext,
    pub kernel_stack: Box<KernelStack>,
    pub user_stack_top: u64,
    pub user_stack_base: u64,
    pub entry_point: u64,
    pub arg: u64,
    pub exit_code: i32,
    pub block_reason: Option<BlockReason>,
    pub wakeup_time: u64,
    pub fs_base: u64,
    pub gs_base: u64,
    pub kernel_gs_base: u64,
    pub fpu_state: Option<Box<FpuState>>,
    pub signal_mask: u64,
    pub pending_signals: u64,
}

impl ThreadControlBlock {
    pub fn new(tid: u64, pid: u64, entry: u64, user_stack: u64, arg: u64) -> Self {
        let mut tcb = Self {
            tid,
            pid,
            name: [0; 32],
            state: ThreadState::Ready,
            context: CpuContext::new(),
            kernel_stack: KernelStack::new(),
            user_stack_top: user_stack,
            user_stack_base: user_stack.saturating_sub(USER_STACK_SIZE as u64),
            entry_point: entry,
            arg,
            exit_code: 0,
            block_reason: None,
            wakeup_time: 0,
            fs_base: 0,
            gs_base: 0,
            kernel_gs_base: 0,
            fpu_state: None,
            signal_mask: 0,
            pending_signals: 0,
        };

        tcb.context.prepare_user_entry(
            entry,
            user_stack,
            USER_CS as u64,
            USER_DS as u64,
            USER_RFLAGS,
        );

        tcb.context.r15 = arg;

        tcb
    }

    pub fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = core::cmp::min(bytes.len(), 31);
        self.name[..len].copy_from_slice(&bytes[..len]);
        self.name[len] = 0;
    }

    pub fn name(&self) -> &str {
        let len = self.name.iter().position(|&c| c == 0).unwrap_or(32);
        core::str::from_utf8(&self.name[..len]).unwrap_or("unknown")
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl InterruptFrame {
    pub fn for_user_entry(entry: u64, stack: u64) -> Self {
        Self { rip: entry, cs: USER_CS as u64, rflags: USER_RFLAGS, rsp: stack, ss: USER_DS as u64 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UserContext {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rax: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
    pub fs_base: u64,
    pub gs_base: u64,
}

#[repr(C)]
pub struct ExecContext {
    pub entry: u64,
    pub stack: u64,
    pub pid: u64,
    pub tid: u64,
    pub cr3: u64,
    pub argc: u64,
    pub argv: u64,
    pub envp: u64,
}
