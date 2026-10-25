// NONOS Operating System (AGPL-3.0-or-later)
//! How a state starts, and what each CPU may do in it.

use super::config::{Config, MAX_CPUS};
use super::state::{Ap, Handler, Init, State};
use super::types::{Step, Val};

impl State {
    pub fn start(cfg: &Config) -> State {
        let mut s = State {
            pte: Val::Old,
            init: Init::Start,
            targets: 0,
            acks: 0,
            tlb: [None; MAX_CPUS],
            ap: [Ap::Offline; MAX_CPUS],
            pending: [false; MAX_CPUS],
            handler: [Handler::Idle; MAX_CPUS],
            park_asked: [false; MAX_CPUS],
        };
        // The initiator, and every AP online at the start, may hold the old
        // translation already.
        s.ap[0] = Ap::Online;
        s.tlb[0] = Some(Val::Old);
        for c in 1..=cfg.online_at_start {
            s.ap[c] = Ap::Online;
            s.tlb[c] = Some(Val::Old);
        }
        s
    }

    /// Runs kernel code on the kernel's page tables, so may fill its TLB.
    pub fn runs_kernel(&self, c: usize) -> bool {
        matches!(self.ap[c], Ap::Woken | Ap::Entered(_) | Ap::Online)
    }

    /// Takes the shootdown IPI, or the NMI the waiter sends at 50 ms.
    pub(crate) fn interruptible(&self, c: usize) -> bool {
        self.ap[c] == Ap::Online
    }

    /// Serves its own pending flag, by interrupt or at a spin point.
    pub(crate) fn serves(&self, c: usize) -> bool {
        matches!(self.ap[c], Ap::Entered(_) | Ap::Online)
    }

    pub(crate) fn descriptor_offline(&self, c: usize, cfg: &Config) -> bool {
        matches!(self.ap[c], Ap::Offline | Ap::Abandoned)
            || (!cfg.drop_after_init && self.park_asked[c] && self.ap[c] != Ap::Online)
    }

    pub(crate) fn descriptor_starting(&self, c: usize, cfg: &Config) -> bool {
        matches!(self.ap[c], Ap::Starting | Ap::Woken | Ap::Entered(_) | Ap::Parked)
            && !self.descriptor_offline(c, cfg)
    }

    /// Holds a lock taken in `bring_up`: an odd number of steps since the
    /// last release.
    pub(crate) fn holds_lock(&self, c: usize, cfg: &Config) -> bool {
        let Ap::Entered(pc) = self.ap[c] else { return false };
        cfg.bring_up[..pc as usize].iter().fold(false, |held, step| match step {
            Step::Lock => true,
            Step::Unlock => false,
            Step::Checkpoint => held,
        })
    }
}
