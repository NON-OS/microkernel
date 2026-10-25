// NONOS Operating System (AGPL-3.0-or-later)

//! The link's power save: asleep or awake, and the watch that decides.

use super::switch::LPS;
use super::watch::{Seen, Watch};
use crate::h2c::H2c;
use crate::pcie::Aspm;
use crate::regs::Mmio;

/// Power save as the link holds it. Built awake, with nothing the host
/// enabled on the PCIe link; `new` takes what it did.
#[derive(Default)]
pub struct Ps {
    pub(super) aspm: Aspm,
    pub(super) asleep: bool,
    pub(super) h2c: H2c,
    watch: Watch,
}

impl Ps {
    pub fn new(aspm: Aspm) -> Self {
        Self { aspm, h2c: H2c::new(), ..Self::default() }
    }

    /// For the proofs: whether the firmware was put to sleep.
    #[cfg(test)]
    pub fn asleep(&self) -> bool {
        self.asleep
    }

    /// The link's clock: at each watch period, sleep or wake as it says.
    /// Nothing at all while the switch is off.
    pub fn tick<M: Mmio>(&mut self, mmio: &M, now_ms: u64, seen: Seen, may_sleep: bool) {
        if LPS {
            self.step(mmio, now_ms, seen, may_sleep);
        }
    }

    /// `tick` regardless of the switch, for the proofs.
    pub fn step<M: Mmio>(&mut self, mmio: &M, now_ms: u64, seen: Seen, may_sleep: bool) {
        match self.watch.due(now_ms, seen, may_sleep) {
            Some(true) if !self.asleep => self.enter(mmio),
            Some(false) => self.wake(mmio),
            _ => {}
        }
    }

    /// Awake now, if asleep: before a teardown, a join or a scan.
    pub fn wake<M: Mmio>(&mut self, mmio: &M) {
        if self.asleep {
            self.leave(mmio);
        }
        self.watch = Watch::default();
    }
}
