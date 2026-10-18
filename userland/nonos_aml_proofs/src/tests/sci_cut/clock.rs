// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's loop on a clock: each tick re-arms what is due, maybe has
//! the source assert, and serves one SCI with the method held to a budget.

use super::fixture::Scene;
use super::run::start;
use crate::events::{Dispatch, SciReport};
use crate::exec::{ExecLimits, Machine};
use crate::load::Aml;
use crate::probe::Probe;
use crate::tests::sci_rig::SciRig;

/// What the loop did at one tick.
pub struct Tick {
    pub rearmed: Vec<u16>,
    pub report: SciReport,
}

pub struct Clock {
    pub rig: SciRig,
    pub aml: Aml,
    pub g: u16,
}

impl Clock {
    pub fn new(sc: &Scene) -> Self {
        let (rig, aml, g) = start(sc);
        Clock { rig, aml, g }
    }

    pub fn tick(&mut self, now: u64, raise: bool, steps: u64) -> Tick {
        self.rig.regs.0.borrow_mut().now = now;
        let mut io = self.rig.regs.clone();
        let rearmed = self.rig.events.rearm_due(&mut io).unwrap();
        if raise {
            self.rig.raise(self.g);
        }
        let aml = &mut self.aml;
        let mut run = |d| {
            let Dispatch::Method(id) = d else { panic!("methods only") };
            let (lim, mut probe) = (ExecLimits { steps, ..ExecLimits::DEFAULT }, Probe::default());
            let mut m = Machine::new(aml, &mut probe, lim);
            m.evaluate(aml, id, vec![]).map(|_| ())
        };
        let report = self.rig.events.on_sci(&mut io, &mut run).unwrap();
        self.aml.rt.notifications.clear();
        Tick { rearmed, report }
    }
}
