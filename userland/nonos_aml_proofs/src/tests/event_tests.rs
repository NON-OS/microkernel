// NONOS Operating System (AGPL-3.0-or-later)
//! Events and Notify.

use crate::consts::at;
use crate::enc::*;
use crate::exec::FaultKind;
use crate::run::{boot, call, call_with, num, Probe};

#[test]
fn events_count_signals() {
    let defs = cat(&[&op(0x5B02), &name("EVT0")]);
    let wait = |t: u64| e(0x5B25, &[&name("EVT0"), &int(t)]);
    let body = cat(&[
        &e(0x5B24, &[&name("EVT0")]),
        &store(&wait(5), &local(0)),
        &store(&wait(5), &local(1)),
        &e(0x5B24, &[&name("EVT0")]),
        &e(0x5B26, &[&name("EVT0")]),
        &store(&wait(5), &local(2)),
        &ret(&cat(&[&[0x72], &local(0), &e(0x72, &[&local(1), &local(2), &[0]]), &[0]])),
    ]);
    let (mut aml, _) =
        boot(&cat(&[&defs, &method("\\M", 0, &body), &method("\\F", 0, &wait(0xFFFF))]), 2);
    let mut probe = Probe::default();
    let r = call_with(&mut aml, &mut probe, Default::default(), "\\M", vec![]).unwrap();
    assert_eq!(num(&r), u64::MAX.wrapping_mul(2), "signalled: 0; unsignalled twice: Ones each");
    assert_eq!(probe.slept_ms, [5, 5]);
    assert_eq!(call(&mut aml, "\\F", vec![]).unwrap_err().kind, FaultKind::Time);
}

#[test]
fn notify_queues_and_counts_what_it_drops() {
    let defs = device("\\_SB.DEV0", &[]);
    let one = e(0x86, &[&name("\\_SB.DEV0"), &int(0x80)]);
    let many = pkg(0xA2, &cat(&[&e(0x95, &[&local(0), &int(70)]), &one, &e(0x75, &[&local(0)])]));
    let body = cat(&[&store(&int(0), &local(0)), &many]);
    let (mut aml, _) = boot(&cat(&[&defs, &method("\\M", 0, &body)]), 2);
    call(&mut aml, "\\M", vec![]).unwrap();
    let dev = at(&aml, "\\_SB.DEV0").unwrap();
    assert_eq!(aml.rt.notifications.len(), 64);
    assert_eq!(aml.rt.notifications[0], (dev, 0x80));
    assert_eq!(aml.rt.dropped_notifications, 6);
}
