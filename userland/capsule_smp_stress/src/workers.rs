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

//! The worker threads: one seat each, so each has its own stack.

use crate::{ipc_pair, pingpong, sleeper, stats};
use core::sync::atomic::{AtomicU32, Ordering};
use nonos_libc::{mk_idle_ms, WorkerSeat};

const IPC_THREADS: usize = ipc_pair::PAIRS * 2;
pub const TOTAL: usize = pingpong::PAIRS * 2 + IPC_THREADS + sleeper::THREADS;

static SEATS: [WorkerSeat; TOTAL] = [const { WorkerSeat::new() }; TOTAL];
/// The kernel's id of each IPC thread, so its partner can address it.
static IPC_TIDS: [AtomicU32; IPC_THREADS] = [const { AtomicU32::new(0) }; IPC_THREADS];

/// Start every worker. On a refusal, the seat it stopped at and the errno.
pub fn spawn_all() -> Result<(), (usize, i64)> {
    let mut seat = 0;
    for side in 0..pingpong::PAIRS * 2 {
        start(&mut seat, pingpong::run, side)?;
    }
    for (side, slot) in IPC_TIDS.iter().enumerate() {
        let tid = start(&mut seat, ipc_pair::run, side)?;
        slot.store(tid, Ordering::Release);
    }
    for n in 0..sleeper::THREADS {
        start(&mut seat, sleeper::run, n)?;
    }
    Ok(())
}

fn start(seat: &mut usize, job: fn(usize), arg: usize) -> Result<u32, (usize, i64)> {
    let tid = SEATS[*seat].spawn(job, arg).map_err(|errno| (*seat, errno))?;
    *seat += 1;
    Ok(tid)
}

/// The id of IPC thread `side`, once its spawn has returned; none on a stop.
pub fn wait_ipc_tid(side: usize) -> Option<u32> {
    loop {
        match IPC_TIDS[side].load(Ordering::Acquire) {
            0 if stats::stopping() => return None,
            0 => {
                mk_idle_ms(1);
            }
            tid => return Some(tid),
        }
    }
}

/// Workers whose thread has not yet ended.
pub fn running() -> usize {
    SEATS.iter().filter(|seat| seat.busy()).count()
}
