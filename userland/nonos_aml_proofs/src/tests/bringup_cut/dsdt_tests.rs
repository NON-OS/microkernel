// NONOS Operating System (AGPL-3.0-or-later)
//! A DSDT whose load-time code the budget cuts, at every step: refused
//! with where; the capsule's rule drops to an empty namespace; and then
//! no \_PIC, _OSC, _PRT or _INI answers anything. The line says the DSDT
//! was cut, where, and that no AML is served. Uncut, all of them answer.

use super::capsule::load_rule::{after_refusal, refused_line, Next};
use super::capsule::text;
use super::fixture::dsdt;
use crate::arena::Limits;
use crate::enc::table;
use crate::error::ErrorKind;
use crate::exec::{ExecLimits, Machine};
use crate::load::Aml;
use crate::namespace::ROOT;
use crate::platform::{initialize, routes, set_pic};
use crate::tests::platform::run::evaluator;

/// The capsule's load (`boot/load.rs`) of the DSDT alone, under `steps`.
fn capsule_load(probe: &mut crate::probe::Probe, steps: u64) -> (Aml, Option<String>, u64) {
    let mut aml = Aml::new(Limits::DEFAULT);
    let mut m = Machine::new(&mut aml, probe, ExecLimits { steps, ..ExecLimits::DEFAULT });
    let r = m.load_table(&mut aml, table(b"DSDT", 2, &dsdt()));
    let used = m.steps();
    let Err(e) = r else { return (aml, None, used) };
    let line = text(|o| refused_line(0, e.kind == ErrorKind::Budget, e.at, o));
    assert_eq!(after_refusal(0), Next::NoAml);
    assert!(e.kind == ErrorKind::Budget && e.at > 0, "{e:?}");
    (Aml::new(Limits::DEFAULT), Some(format!("{line}|{:#x}", e.at)), used)
}

#[test]
fn a_cut_dsdt_is_never_served() {
    let mut probe = crate::probe::Probe::default();
    let (mut full, said, used) = capsule_load(&mut probe, 1_000_000);
    assert!(said.is_none() && used > 0);
    let pci = |a: &Aml| a.ns.child(ROOT, *b"_SB_").and_then(|sb| a.ns.child(sb, *b"PCI0"));
    let bridge = pci(&full).unwrap();
    let want = routes(&mut full, &mut evaluator(&mut probe), bridge);
    assert!(matches!(want, Some(Ok(ref r)) if r.len() == 2));
    assert!(used >= 3, "the DSDT's load-time code takes steps to cut: {used}");
    for steps in 1..used {
        let (mut aml, said, _) = capsule_load(&mut probe, steps);
        let said = said.expect("refused");
        let (line, at) = said.split_once('|').unwrap();
        let stands = "the kernel's MADT routing and scanned \\_S5 stand";
        assert_eq!(line, format!("DSDT #0 cut short at {at}; no AML is served: {stands}"));
        assert!(pci(&aml).is_none(), "cut at {steps}: no host bridge to route");
        assert!(set_pic(&mut aml, &mut evaluator(&mut probe), 1).is_none(), "no \\_PIC");
        let ini = initialize(&mut aml, &mut evaluator(&mut probe));
        assert!(ini.ran.is_empty() && ini.sta_failed.is_empty(), "no _INI");
        assert!(aml.ns.child(ROOT, *b"_SB_").and_then(|sb| aml.ns.child(sb, *b"_OSC")).is_none());
    }
}
