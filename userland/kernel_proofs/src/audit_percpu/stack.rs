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

//! A ring is never built on the stack of the CPU that records into it.
//!
//! The kernel records each syscall from the caller's 32 KiB kernel stack, and
//! those stacks sit next to each other with no guard page between them. A ring
//! of the kernel's records is 22 KiB: built on that stack and then moved to
//! the heap, a CPU's first record ran off the bottom of the stack it was on
//! and over the top of the next process's, where that process's saved return
//! addresses are. So a CPU's first record, and a
//! clear, must take stack that does not grow with the ring.
//!
//! Each case runs in a child of this test binary, on a thread whose stack is a
//! kernel stack's size, holding records far larger than a kernel record, so a
//! ring built on that stack overflows it whatever the optimiser copies. The
//! host's guard page then ends the child, and the parent says so, instead of
//! the whole test binary aborting.

use std::process::Command;
use std::thread;

use super::percpu::{PerCpuLog, RING};

/// The kernel's per-process kernel stack (process::userspace::constants).
const KERNEL_STACK: usize = 32 * 1024;

/// The variable that tells a child which case to run.
const CASE: &str = "NONOS_KP_AUDIT_STACK_CASE";

/// A record whose ring, `RING` of them, is several kernel stacks.
#[derive(Clone)]
struct Big([u64; 128]);

const _: () = assert!(RING * core::mem::size_of::<Big>() > 4 * KERNEL_STACK);

/// The kernel's record (syscall::dispatch::audit::entry), field for field: the
/// ring the kernel holds is this size, which is two thirds of a kernel stack.
#[derive(Clone)]
struct KernelRecord {
    timestamp_ms: u64,
    syscall_num: u64,
    syscall_name: &'static str,
    pid: u32,
    result: i64,
    args: [u64; 4],
    success: bool,
}

#[test]
fn the_kernels_ring_is_two_thirds_of_a_kernel_stack() {
    let ring = RING * core::mem::size_of::<Option<(u64, KernelRecord)>>();
    assert_eq!(ring, 22 * 1024);
    assert!(ring * 3 >= KERNEL_STACK * 2, "{ring} bytes of a {KERNEL_STACK}-byte stack");
}

/// Runs `case` in a child of this binary on a kernel-stack-sized thread.
fn in_child(name: &str, case: &str) {
    let exe = std::env::current_exe().expect("the test binary");
    let out = Command::new(exe)
        .args(["--exact", name, "--test-threads=1", "--nocapture"])
        .env(CASE, case)
        .output()
        .expect("run the child");
    assert!(
        out.status.success(),
        "{case} on a {KERNEL_STACK}-byte stack: {}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
}

/// In the child: `body` on a thread with a kernel stack's room. True when this
/// process is the child, which has then run the case.
fn on_kernel_stack(case: &str, body: fn()) -> bool {
    if std::env::var(CASE).as_deref() != Ok(case) {
        return false;
    }
    thread::Builder::new()
        .stack_size(KERNEL_STACK)
        .spawn(body)
        .expect("a thread on a kernel-stack-sized stack")
        .join()
        .expect("the case");
    true
}

#[test]
fn a_cpus_first_record_takes_no_ring_sized_stack() {
    const NAME: &str = "audit_percpu::stack::a_cpus_first_record_takes_no_ring_sized_stack";
    if on_kernel_stack("first", || {
        let log: PerCpuLog<Big, 4> = PerCpuLog::new();
        for cpu in 0..4 {
            log.push(cpu, Big([cpu as u64; 128]));
        }
        let firsts: Vec<u64> = log.recent(8).iter().map(|r| r.0[127]).collect();
        assert_eq!(firsts, vec![3, 2, 1, 0]);
    }) {
        return;
    }
    in_child(NAME, "first");
}

#[test]
fn a_clear_takes_no_ring_sized_stack() {
    const NAME: &str = "audit_percpu::stack::a_clear_takes_no_ring_sized_stack";
    if on_kernel_stack("clear", || {
        let log: PerCpuLog<Big, 2> = PerCpuLog::new();
        for i in 0..(RING as u64 + 3) {
            log.push((i % 2) as usize, Big([i; 128]));
        }
        log.clear();
        assert!(log.recent(usize::MAX).is_empty());
        log.push(1, Big([7; 128]));
        let left: Vec<u64> = log.recent(4).iter().map(|r| r.0[0]).collect();
        assert_eq!(left, vec![7]);
    }) {
        return;
    }
    in_child(NAME, "clear");
}

#[test]
fn kernel_records_from_kernel_stacks_read_back_whole() {
    const NAME: &str = "audit_percpu::stack::kernel_records_from_kernel_stacks_read_back_whole";
    if on_kernel_stack("kernel", || {
        let log: PerCpuLog<KernelRecord, 4> = PerCpuLog::new();
        for cpu in 0..4u64 {
            let record = KernelRecord {
                timestamp_ms: 1000 + cpu,
                syscall_num: 0x100 + cpu,
                syscall_name: "MkIpcCall",
                pid: cpu as u32 + 3,
                result: -(cpu as i64),
                args: [cpu; 4],
                success: cpu == 0,
            };
            assert!(log.push(cpu as usize, record));
        }
        let back: Vec<_> = log
            .recent(8)
            .into_iter()
            .map(|KernelRecord { timestamp_ms, syscall_num, syscall_name, pid, result, args, success }| {
                (timestamp_ms, syscall_num, syscall_name, pid, result, args, success)
            })
            .collect();
        let want: Vec<_> = (0..4u64)
            .rev()
            .map(|c| (1000 + c, 0x100 + c, "MkIpcCall", c as u32 + 3, -(c as i64), [c; 4], c == 0))
            .collect();
        assert_eq!(back, want);
    }) {
        return;
    }
    in_child(NAME, "kernel");
}
