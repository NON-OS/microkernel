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

//! The write and the read-back, in steps, with a line every ten percent so
//! a serial log of the run stays short. On failure the exit code the
//! command should end with, the reason and the disk's state already printed.

use nonos_blk_client::{BlockDevice, DeviceSink};
use nonos_disk::{Plan, Progress, Receipt, Session, Verifier};

const STEP: usize = 4 << 20;

pub fn write_and_verify(device: BlockDevice, plan: Plan<'_>) -> Result<(Receipt<'_>, u64), i32> {
    let mut sink = DeviceSink { device };
    let mut session = Session::new(plan);
    let receipt = loop {
        match session.step(&mut sink, STEP) {
            Ok(Progress::Writing { done, total }) => tick(done, total, "writing"),
            Ok(Progress::TableWritten) => println!("partition table written"),
            Ok(Progress::Done(r)) => break r,
            Err(e) => {
                eprintln!("install: write failed: {e:?}; the disk has no partition table");
                return Err(3);
            }
        }
    };
    let mut v = Verifier::new(&receipt);
    let total = v.total_bytes();
    loop {
        match v.step(&mut sink, STEP) {
            Ok(true) => tick(v.checked, total, "reading back"),
            Ok(false) => break,
            Err(e) => {
                eprintln!("install: read-back failed: {e:?}; do not boot this disk");
                return Err(4);
            }
        }
    }
    Ok((receipt, v.checked))
}

fn tick(done: u64, total: u64, what: &str) {
    if total == 0 {
        return;
    }
    let pct = done * 100 / total;
    let before = done.saturating_sub(STEP as u64) * 100 / total;
    if pct / 10 != before / 10 || done == total {
        println!("{what} {pct:>3}%");
    }
}
