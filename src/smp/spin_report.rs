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

//! The line a lock wait too long prints: where it waits, on which CPU and for
//! whom, and where the lock was taken.

use core::panic::Location;

use super::spin_holders::taker;
use super::spin_verdict::{judge, Holder};

/// The lines for a wait on `lock` at `site` that has lasted `ms`: where it
/// waits, where the lock was taken, and, for a re-take, the verdict. Each is
/// its own line, short of the serial line's 192 bytes, and each says its CPU,
/// since another CPU's lines may fall between them. It is bounded and takes
/// only the serial lock, which nothing takes through `lock_responsive` and
/// whose holders only write bytes; it allocates nothing.
pub(super) fn report(lock: usize, site: &'static Location<'static>, sp: usize, ms: u64) {
    let cpu = super::cpu_id::try_cpu_id();
    let held =
        taker(lock).map(|(at, held_sp, shared)| (at, judge(held_sp, sp, stack_top(cpu), shared)));
    let n = cpu.map_or(u64::MAX, |cpu| cpu as u64);
    let mut line = crate::sys::serial::Line::new();
    line.str(b"[LOCK] cpu=").dec(n).str(b" waiting ms=").dec(ms).str(b" pid=");
    line.hex(crate::process::current_pid().unwrap_or(0) as u64).str(b" at ");
    line.str(site.file().as_bytes()).str(b":").dec(site.line() as u64).end();
    let mut line = crate::sys::serial::Line::new();
    line.str(b"[LOCK] cpu=").dec(n);
    match held {
        Some((at, verdict)) => {
            line.str(b" held from ").str(at.file().as_bytes()).str(b":").dec(at.line() as u64);
            if verdict == Holder::Likely {
                line.str(b", likely re-taken by its own holder");
            }
        }
        None => _ = line.str(b" holder unknown"),
    }
    line.end();
    if matches!(held, Some((_, Holder::ReTaken))) {
        let mut line = crate::sys::serial::Line::new();
        line.str(b"[FATAL] cpu=")
            .dec(n)
            .str(b" lock re-taken by its own holder, cannot go on")
            .end();
    }
}

/// The top of the kernel stack this CPU runs on, 0 when unknown. Only the
/// x86_64 switch path writes it; elsewhere it stays 0, which `judge` reads
/// as not known.
fn stack_top(cpu: Option<usize>) -> usize {
    cpu.and_then(super::percpu::get).map_or(0, |data| data.kernel_stack_top as usize)
}
