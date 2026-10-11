// NONOS Operating System (AGPL-3.0-or-later)
//! Recursion, allocation and waiting, each stopped by its budget.

use crate::enc::*;
use crate::exec::{ExecLimits, FaultKind};
use crate::object::Object;
use crate::run::{boot, call_with, num, Probe};

fn limits(steps: u64, call_depth: u32, alloc: usize, time_ms: u64) -> ExecLimits {
    ExecLimits { steps, call_depth, alloc, time_ms }
}

fn run(body: &[u8], lim: ExecLimits) -> Result<Object, crate::exec::Fault> {
    let (mut aml, _) = boot(&method("\\M", 0, body), 2);
    call_with(&mut aml, &mut Probe::default(), lim, "\\M", vec![])
}

#[test]
fn recursion_stops_at_the_call_depth() {
    let rec = method("\\R", 0, &ret(&name("R")));
    let (mut aml, _) = boot(&rec, 2);
    let r =
        call_with(&mut aml, &mut Probe::default(), limits(1 << 20, 6, 1 << 20, 0), "\\R", vec![]);
    assert_eq!(r.unwrap_err().kind, FaultKind::CallDepth);
    let down = e(0x74, &[&arg(0), &int(1), &[0]]);
    let n =
        method("\\N", 1, &cat(&[&if_(&arg(0), &ret(&cat(&[&name("N"), &down]))), &ret(&int(9))]));
    let (mut aml, _) = boot(&n, 2);
    let ok = call_with(
        &mut aml,
        &mut Probe::default(),
        limits(1 << 20, 6, 1 << 20, 0),
        "\\N",
        vec![Object::Integer(5)],
    );
    assert_eq!(num(&ok.unwrap()), 9, "six calls deep is within a depth of six");
    let deep = call_with(
        &mut aml,
        &mut Probe::default(),
        limits(1 << 20, 6, 1 << 20, 0),
        "\\N",
        vec![Object::Integer(6)],
    );
    assert_eq!(deep.unwrap_err().kind, FaultKind::CallDepth);
}

#[test]
fn allocation_and_time_are_bounded() {
    let grow = cat(&[
        &store(&string("ab"), &local(0)),
        &pkg(0xA2, &cat(&[&int(1), &e(0x73, &[&local(0), &local(0), &local(0)])])),
    ]);
    assert_eq!(run(&grow, limits(1 << 30, 24, 1 << 16, 0)).unwrap_err().kind, FaultKind::Alloc);
    let nap = cat(&[&e(0x5B22, &[&int(600)]), &e(0x5B22, &[&int(600)])]);
    let (mut aml, _) = boot(&method("\\M", 0, &nap), 2);
    let mut probe = Probe::default();
    let r = call_with(&mut aml, &mut probe, limits(1 << 20, 24, 1 << 20, 1000), "\\M", vec![]);
    assert_eq!(r.unwrap_err().kind, FaultKind::Time);
    assert_eq!(probe.slept_ms, [600], "the second sleep was refused before it began");
}
