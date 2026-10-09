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

//! Where a worker starts. The kernel enters it at the trampoline with nothing
//! in its registers and its stack pointer where the spawner put it; the
//! spawner left the job and its argument just above (`Start`), so the
//! trampoline hands their address to Rust as the first argument.

/// What a worker runs, written near the top of its stack before it is spawned.
#[repr(C)]
pub(super) struct Start {
    pub(super) job: fn(usize),
    pub(super) arg: usize,
}

/// Bytes from the worker's first stack pointer up to `Start`. The spawner
/// keeps both 16-aligned, so the stack is aligned at the trampoline's call
/// as the SysV ABI wants.
pub(super) const START_GAP: usize = 16;

/// The address of `Start`, then into Rust. `start` never returns; the `ud2`
/// is there should it ever.
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
pub(super) unsafe extern "C" fn trampoline() -> ! {
    core::arch::naked_asm!(
        "lea rdi, [rsp + {gap}]",
        "call {start}",
        "ud2",
        gap = const START_GAP,
        start = sym start,
    );
}

/// Run the job, then end this thread. `MkExit` from a thread ends only that
/// thread; its process and the process's other threads go on.
#[cfg(target_arch = "x86_64")]
extern "C" fn start(at: usize) -> ! {
    // SAFETY: `at` is the `Start` the spawner wrote above this thread's first
    // stack pointer before spawning it. Nothing writes it after, and it is
    // read once, here, before the job's frames grow below it.
    let Start { job, arg } = unsafe { core::ptr::read(at as *const Start) };
    job(arg);
    crate::unistd::mk_exit(0)
}
