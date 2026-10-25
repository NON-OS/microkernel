// NONOS Operating System (AGPL-3.0-or-later)
//! One SCI with its GPE method held to a step budget, recording the GPE's
//! register writes before and after the method; the drain, each delivery's
//! HND held the same; and a started rig for a scene.

use super::fixture::{dsdt, Scene};
use crate::events::{Dispatch, Policy, SciReport};
use crate::exec::{ExecLimits, Machine};
use crate::fadt_enc::GPE0;
use crate::load::Aml;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::sci_rig::SciRig;

fn lim(steps: u64) -> ExecLimits {
    ExecLimits { steps, ..ExecLimits::DEFAULT }
}

/// One SCI: the report, the GPE's writes before and after its method, and
/// the steps the method used.
pub fn sci(
    rig: &mut SciRig,
    aml: &mut Aml,
    probe: &mut Probe,
    g: u16,
    steps: u64,
) -> (SciReport, [Vec<(u16, u16)>; 2], u64) {
    rig.regs.0.borrow_mut().writes.clear();
    rig.raise(g);
    let (regs, mut mark, mut used) = (rig.regs.clone(), 0, 0);
    let mut run = |d| {
        let Dispatch::Method(id) = d else { panic!("methods only") };
        mark = regs.0.borrow().writes.len();
        let mut m = Machine::new(aml, probe, lim(steps));
        let r = m.evaluate(aml, id, vec![]).map(|_| ());
        used = m.steps();
        r
    };
    let mut io = rig.regs.clone();
    let report = rig.events.on_sci(&mut io, &mut run).unwrap();
    let mine = |w: &[(u16, u16)]| {
        w.iter().copied().filter(|(p, _)| *p == GPE0 || *p == GPE0 + 8).collect()
    };
    let w = rig.regs.0.borrow().writes.clone();
    (report, [mine(&w[..mark]), mine(&w[mark..])], used)
}

pub fn drain(aml: &mut Aml, probe: &mut Probe, steps: u64) -> Vec<u8> {
    let mut out = Vec::new();
    while let Some((id, v)) = aml.rt.notifications.pop_front() {
        out.push(v);
        let hnd = aml.ns.child(id, *b"HND_").unwrap();
        let _ = Machine::new(aml, probe, lim(steps)).evaluate(aml, hnd, vec![]);
    }
    out
}

pub fn start(sc: &Scene) -> (SciRig, Aml, u16) {
    let (aml, _) = boot(&dsdt(sc), 2);
    let mut rig = SciRig::new(&aml, &[], Policy::default());
    rig.start();
    (rig, aml, if sc.level { 1 } else { 2 })
}
