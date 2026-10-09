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

//! The enable and disable waits: after writing CC.EN the driver polls CSTS
//! until RDY follows, for as long as CAP.TO (in 500 ms units) allows. CAP and
//! CSTS are the controller's to report, so the proofs run the driver's own
//! loop against scripted controllers on a scripted clock: a fatal status, an
//! all-ones read (a device pulled from the bus) and a register that never
//! flips all end the wait with an error, in bounded time, and never as a
//! success, whatever CAP.TO says.

use core::cell::Cell;

use crate::admin::{ready_step, ready_timeout_ms, wait_ready, ReadyStep, READY_FLOOR_MS};
use crate::constants::{CSTS_CFS, CSTS_RDY};
use crate::error::{NvmeError, NvmeResult};

/// The spec's largest worst case: CAP.TO is eight bits of 500 ms units.
pub(crate) const SPEC_CEILING_MS: u64 = 255 * 500;

/// More CSTS reads than any bounded wait here needs; reaching it means the
/// loop did not end, and the scripted controller says so instead of hanging.
const READ_CEILING: u64 = 50_000_000;

pub(crate) struct Run {
    pub result: NvmeResult<()>,
    pub elapsed_ms: u64,
    pub reads: u64,
}

pub(crate) fn cap_with_to(cap: u64, to: u8) -> u64 {
    (cap & !(0xff << 24)) | ((to as u64) << 24)
}

/// Run the driver's wait against a controller with this CAP whose CSTS reads
/// `csts(ms)` at simulated time `ms`, on a clock that moves `tick` ms each
/// time it is read.
pub(crate) fn run(cap: u64, want_ready: bool, tick: u64, csts: impl Fn(u64) -> u32) -> Run {
    const START: u64 = 1_000_000;
    let now = Cell::new(START);
    let reads = Cell::new(0u64);
    let result = wait_ready(
        cap,
        want_ready,
        || {
            reads.set(reads.get() + 1);
            assert!(reads.get() < READ_CEILING, "the ready wait did not end");
            csts(now.get() - START)
        },
        || {
            let t = now.get();
            now.set(t + tick);
            Some(t)
        },
    );
    Run { result, elapsed_ms: now.get() - START, reads: reads.get() }
}

fn rdy(ready: bool) -> u32 {
    if ready {
        CSTS_RDY
    } else {
        0
    }
}

const EDGE_TO: [u8; 9] = [0, 1, 9, 10, 11, 20, 128, 254, 255];

#[test]
fn the_timeout_is_cap_to_and_never_under_the_floor_or_over_the_spec() {
    let mut s = 0x6a09_e667_f3bc_c909u64;
    for to in 0..=255u8 {
        for _ in 0..64 {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            let t = ready_timeout_ms(cap_with_to(s, to));
            assert_eq!(t, (to as u64 * 500).max(READY_FLOOR_MS), "CAP.TO {to}");
            assert!((READY_FLOOR_MS..=SPEC_CEILING_MS).contains(&t));
        }
    }
}

#[test]
fn a_fatal_status_ends_the_enable_wait_at_the_first_read() {
    for to in EDGE_TO {
        for csts in [CSTS_CFS, CSTS_CFS | CSTS_RDY, CSTS_CFS | 0xffff_0000] {
            let r = run(cap_with_to(0, to), true, 1, |_| csts);
            assert!(
                matches!(r.result, Err(NvmeError::UnsupportedController)),
                "CSTS {csts:#x} waiting for RDY=1, CAP.TO {to}"
            );
            assert_eq!(r.reads, 1);
        }
    }
}

/*
 * Clearing CC.EN is the reset that clears CFS. The disable wait refused a
 * controller at its first read of CFS, so one that firmware or an earlier
 * boot left fatal was never reset and never served.
 */
#[test]
fn the_disable_wait_resets_a_fatal_controller_and_waits_for_cfs_to_clear() {
    for to in EDGE_TO {
        let r = run(cap_with_to(0, to), false, 1, |n| if n < 5 { CSTS_CFS | CSTS_RDY } else { 0 });
        assert!(r.result.is_ok(), "the reset cleared CFS and RDY, CAP.TO {to}");
        let r = run(cap_with_to(0, to), false, 1, |_| CSTS_CFS);
        assert!(
            matches!(r.result, Err(NvmeError::ControllerFatal)),
            "CFS that never clears is waited out and named fatal, not slow, CAP.TO {to}"
        );
        assert!(r.elapsed_ms >= ready_timeout_ms(cap_with_to(0, to)));
    }
}

#[test]
fn an_all_ones_csts_is_neither_ready_nor_disabled() {
    for cap in [0, u64::MAX, cap_with_to(0, 255)] {
        for want in [true, false] {
            let r = run(cap, want, 1, |_| u32::MAX);
            assert!(r.result.is_err(), "all ones passed for RDY={want}");
            assert_eq!(r.reads, 1, "a pulled device must not be waited on");
            assert!(matches!(ready_step(cap, u32::MAX, want, 0), ReadyStep::Failed(_)));
        }
    }
}

#[test]
fn a_csts_that_never_flips_times_out_within_the_bound() {
    for to in EDGE_TO {
        let cap = cap_with_to(0x0000_00ff_00ff_ffff, to);
        let bound = ready_timeout_ms(cap);
        for want in [true, false] {
            for tick in [1, 7, 499, 5_000, 1 << 40] {
                let r = run(cap, want, tick, |_| rdy(!want));
                assert!(matches!(r.result, Err(NvmeError::ControllerTimeout)), "RDY={want}");
                // The loop reads the clock twice before its first decision,
                // so it ends within two ticks of the bound and not before.
                assert!(r.elapsed_ms >= bound, "CAP.TO {to}: gave up at {}", r.elapsed_ms);
                assert!(r.elapsed_ms <= bound + 2 * tick, "CAP.TO {to}: ran to {}", r.elapsed_ms);
                assert!(bound <= SPEC_CEILING_MS);
            }
        }
    }
}

#[test]
fn a_slow_controller_gets_all_of_its_cap_to() {
    // (CAP.TO, when RDY follows CC.EN in ms)
    for (to, flips_at) in [(20u8, 9_000u64), (60, 29_999), (255, 127_499)] {
        for want in [true, false] {
            let r = run(cap_with_to(0, to), want, 1, |ms| {
                rdy(if ms >= flips_at { want } else { !want })
            });
            assert!(r.result.is_ok(), "CAP.TO {to}, ready at {flips_at} ms, RDY={want}");
        }
    }
}

#[test]
fn a_zero_cap_to_still_gets_the_floor() {
    for want in [true, false] {
        let flips_at = READY_FLOOR_MS - 1;
        let r = run(cap_with_to(u64::MAX, 0), want, 1, |ms| {
            rdy(if ms >= flips_at { want } else { !want })
        });
        assert!(r.result.is_ok(), "RDY={want}");
        assert!(r.elapsed_ms <= flips_at + 2);
        let late = run(cap_with_to(u64::MAX, 0), want, 1, |ms| {
            rdy(if ms > READY_FLOOR_MS { want } else { !want })
        });
        assert!(matches!(late.result, Err(NvmeError::ControllerTimeout)));
    }
}

#[test]
fn the_decision_never_waits_past_the_bound_or_takes_a_wrong_csts() {
    let mut s = 0x2545_f491_4f6c_dd1du64;
    for _ in 0..200_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let cap = s.rotate_left(17);
        let csts = s as u32;
        let want = s & (1 << 40) != 0;
        let elapsed = if s & (1 << 41) != 0 { (s >> 42) % 140_000 } else { s >> 20 };
        match ready_step(cap, csts, want, elapsed) {
            ReadyStep::Reached => {
                assert_eq!(csts & CSTS_CFS, 0);
                assert_eq!(csts & CSTS_RDY != 0, want);
            }
            ReadyStep::Wait => assert!(elapsed < ready_timeout_ms(cap).min(SPEC_CEILING_MS)),
            ReadyStep::Failed(_) => {}
        }
    }
}
