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

//! One bring-up attempt on a `board`, through the entry the capsule's start
//! calls (`setup::run`), with every broker call it makes watched.

use nonos_libc::{calls, logged, watch, Call};

use crate::discover::{Found, Survey};
use crate::error::HdaResult;
use crate::model::WINDOW;
use crate::proofs::lifecycle::board::{Board, DEVICE};
use crate::proofs::lifecycle::teardown_log::snapshot;
use crate::setup::{run, Started};

/// QEMU's `intel-hda`, 8086:2668: Intel, so the position buffer is used.
pub fn ich6() -> Found {
    let (device_id, vendor, device, bar_size) = (DEVICE, 0x8086, 0x2668, WINDOW as u64);
    Found { device_id, vendor, device, subclass: 3, irq_pin: 1, irq_line: 0xff, bar_size }
}

pub fn survey() -> Survey {
    Survey { hda: [Some(ich6()), None, None, None], amd_acp: false, intel_sst: None }
}

/// Run one attempt over `b` and return how it ended and the broker calls
/// it made, each with the registers as they stood then.
pub fn attempt(b: &Board) -> (HdaResult<Started>, Vec<Call>) {
    watch(snapshot(&b.sim.bar));
    let before = calls().len();
    let r = run(&survey(), &mut None);
    (r, calls().split_off(before))
}

/// `attempt` on boards from `make` until one ends as `ended` expects, given
/// how it ended and the lines it logged. The codecs answer from a thread,
/// and one starved past a verb's deadline ends an attempt some other way,
/// which proves nothing about the way under test.
pub fn attempt_until(
    make: impl Fn() -> Board,
    ended: fn(&HdaResult<Started>, &[String]) -> bool,
) -> (Board, HdaResult<Started>, Vec<Call>) {
    for _ in 0..5 {
        let (b, from) = (make(), logged().len());
        let (r, log) = attempt(&b);
        if ended(&r, &logged()[from..]) {
            return (b, r, log);
        }
    }
    panic!("in five attempts none ended the way under test");
}
