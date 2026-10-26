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

//! The proofs on a worker thread, so the serving thread answers STATUS and
//! GET at once throughout them (I-B9a: a STATUS waited out a whole isolation
//! proof, minutes, and the store's window froze with it). The worker makes
//! no IPC or service call: the serving thread gathered all of it first
//! (device.rs, isolation.rs). Its stack is libc's WorkerSeat's, above an
//! unmapped guard page.

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_libc::thread::{WorkerSeat, WORKER_STACK};
use nonos_proof_abi::wire::Stage;

use super::progress::Progress;
use super::status::failed;
use super::work::{Made, Work};
use super::{device, isolation};

/// The deepest stack the proofs reach, painted and measured on the host at
/// opt-level 2 on one core: nonos-device-attest 16 512 bytes, isolation_stark
/// 13 712. The stack holds twice it and more, beside the guard.
const PEAK: usize = 16_512;
const _: () = assert!(WORKER_STACK >= 2 * PEAK);

/// The job's progress: a prover process runs one job and exits.
pub static PROGRESS: Progress<Result<Made, i32>> = Progress::new();
static SEAT: WorkerSeat = WorkerSeat::new();

/// Serving thread: start the worker on `work`, or its failure status.
pub fn start(work: Work) -> Result<(), i32> {
    let arg = Box::into_raw(Box::new(work)) as usize;
    SEAT.spawn(run, arg).map(|_| ()).map_err(|rc| {
        // SAFETY: no thread started, so the box is still this thread's alone.
        drop(unsafe { Box::from_raw(arg as *mut Work) });
        failed(rc)
    })
}

fn run(arg: usize) {
    // SAFETY: `start` handed this thread the box and kept no copy of it.
    let work = unsafe { Box::from_raw(arg as *mut Work) };
    let made = make(*work);
    PROGRESS.finish(made.is_ok(), made);
}

fn make(work: Work) -> Result<Made, i32> {
    let Work { device, isolation } = work;
    PROGRESS.advance(Stage::Device);
    let device = device::prove(device)?;
    let Some(i) = isolation else {
        return Ok(Made { device, isolation: Vec::new() });
    };
    PROGRESS.advance(Stage::Isolation);
    Ok(Made { device, isolation: isolation::prove(i)? })
}
