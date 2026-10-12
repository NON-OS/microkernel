// NONOS Operating System (AGPL-3.0-or-later)
//! The step budget: a loop inside it runs to its end, one step more is
//! refused, and a loop that never ends stops.

use crate::enc::*;
use crate::exec::{ExecLimits, FaultKind};
use crate::object::Object;
use crate::run::{boot, call_with, num, Probe};

fn limits(steps: u64, call_depth: u32, alloc: usize, time_ms: u64) -> ExecLimits {
    ExecLimits { steps, call_depth, alloc, time_ms, ..ExecLimits::DEFAULT }
}

/// A loop of `n` passes, counting in Local0.
fn counted(n: u64) -> Vec<u8> {
    let body = e(0x75, &[&local(0)]);
    cat(&[
        &store(&int(0), &local(0)),
        &pkg(0xA2, &cat(&[&e(0x95, &[&local(0), &int(n)]), &body])),
        &ret(&local(0)),
    ])
}

fn run(body: &[u8], lim: ExecLimits) -> Result<Object, crate::exec::Fault> {
    let (mut aml, _) = boot(&method("\\M", 0, body), 2);
    call_with(&mut aml, &mut Probe::default(), lim, "\\M", vec![])
}

#[test]
fn a_loop_runs_exactly_to_its_step_budget() {
    let full = ExecLimits::DEFAULT;
    // Find the cost of 100 passes, then show one step less is refused.
    let (mut lo, mut hi) = (1u64, 100_000u64);
    while lo < hi {
        let mid = (lo + hi) / 2;
        match run(&counted(100), limits(mid, 24, full.alloc, 0)) {
            Ok(v) => {
                assert_eq!(num(&v), 100);
                hi = mid
            }
            Err(f) => {
                assert_eq!(f.kind, FaultKind::Steps);
                lo = mid + 1
            }
        }
    }
    let cost = lo;
    assert!(run(&counted(100), limits(cost, 24, full.alloc, 0)).is_ok());
    assert!(run(&counted(101), limits(cost, 24, full.alloc, 0)).is_err());
}

#[test]
fn a_loop_that_never_ends_stops() {
    let forever = pkg(0xA2, &cat(&[&int(1), &[0xA3]]));
    assert_eq!(run(&forever, ExecLimits::DEFAULT).unwrap_err().kind, FaultKind::Steps);
}
