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

//! Fault containment, run last because it ends the capsule.
//!
//! First a loop that never yields: while it holds a CPU, the rest of the
//! system must go on making syscalls, which the machine's syscall total in
//! `MkProcStat` shows. Then a write to the null page, which nothing maps: the
//! kernel must end this capsule and nothing else. A capsule cannot report its
//! own death, so it names the fault before it takes it, and the checker
//! (`nonos-ci/attack_suite_check.py`) finds the kernel's `[TRAP PF]` line for
//! this pid and that the log went on after it.

use nonos_libc::{mk_exit, mk_getpid, mk_proc_stat, mk_uptime_ms, ProcStatEntry, ProcStatHeader};

use crate::line::{counted, marker, note, verdict};

/// How long the loop holds its CPU.
const SPIN_MS: i64 = 3000;
/// Iterations between two looks at the clock, the loop's only syscall.
const CHECK_EVERY: u32 = 1 << 20;
/// The address the fault writes: the null page, never mapped.
const NOWHERE: u64 = 0;

#[repr(C)]
#[derive(Default)]
struct Snapshot {
    head: ProcStatHeader,
    first: ProcStatEntry,
}

/// Every syscall the machine has taken, this capsule's included.
fn syscalls() -> Option<u64> {
    let mut s = Snapshot::default();
    match mk_proc_stat(&mut s as *mut Snapshot as *mut u8, 1) {
        n if n >= 1 => Some(s.head.syscalls),
        _ => None,
    }
}

/// Hold the CPU for `SPIN_MS` without yielding, then count what the others
/// did meanwhile.
pub fn spin() {
    let Some(before) = syscalls() else {
        note(b"[ATTACK-NOTE] fault-containment: MkProcStat gave no header, the loop was not tried\n");
        return;
    };
    let start = mk_uptime_ms();
    // The clock reads and the second MkProcStat are this capsule's own.
    let mut mine: u64 = 2;
    let mut n: u32 = 0;
    loop {
        n = core::hint::black_box(n.wrapping_add(1));
        if n % CHECK_EVERY == 0 {
            mine += 1;
            if mk_uptime_ms() - start >= SPIN_MS {
                break;
            }
        }
    }
    let Some(after) = syscalls() else {
        note(b"[ATTACK-NOTE] fault-containment: MkProcStat gave no header after the loop\n");
        return;
    };
    match after.wrapping_sub(before).saturating_sub(mine) {
        0 => note(b"[ATTACK-NOTE] fault-containment: nothing else made a syscall while it looped, the loop half is unproven on this boot\n"),
        others => counted(
            b"fault-containment",
            b"a loop that never yields held no CPU for 3000 ms, the others made",
            others,
            b"syscalls meanwhile",
        ),
    }
}

/// Write to the null page. The kernel ends the capsule here; the lines
/// after the write print only if it did not.
pub fn fault() -> ! {
    marker(b"[ATTACK-FAULT] pid ", u64::from(mk_getpid()), b" writes address 0x0\n");
    // SAFETY: the write is the attack. It faults, and the kernel ends this
    // capsule before the instruction completes.
    unsafe {
        core::arch::asm!(
            "mov qword ptr [{at}], {v}",
            at = in(reg) NOWHERE,
            v = in(reg) 0x4E4F_4E4F_5345_4743u64,
            options(nostack),
        );
    }
    verdict(b"fault-containment", b"a write to the null page", 0);
    mk_exit(1)
}
