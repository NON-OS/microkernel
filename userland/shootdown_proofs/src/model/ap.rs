// NONOS Operating System (AGPL-3.0-or-later)
//! AP bring-up, and the boot CPU giving up on or parking an AP.

use super::config::Config;
use super::state::{Ap, State};
use super::types::Step;

pub(super) fn step(s: &State, cfg: &Config, out: &mut Vec<State>) {
    let n = cfg.cpus;
    // AP bring-up, and the boot CPU giving up on an AP that never claimed.
    for c in 1..n {
        let mut t = *s;
        match s.ap[c] {
            Ap::Offline => t.ap[c] = Ap::Starting,
            Ap::Starting => {
                // INIT empties the TLB.
                t.tlb[c] = None;
                t.ap[c] = Ap::Woken;
            }
            Ap::Woken => t.ap[c] = Ap::Entered(0),
            Ap::Entered(pc) => {
                let pc = pc as usize;
                // With no park request possible the steps of `bring_up` are
                // all alike to the round, so one step stands for them.
                if pc == cfg.bring_up.len() || !cfg.may_ask_park {
                    // `go_live`: interrupts on, `Online` published.
                    t.ap[c] = Ap::Online;
                } else if cfg.bring_up[pc] == Step::Checkpoint && s.park_asked[c] {
                    // CAS to `Parked`, then `cli; hlt`.
                    t.ap[c] = Ap::Parked;
                } else {
                    t.ap[c] = Ap::Entered(pc as u8 + 1);
                }
            }
            Ap::Parked => {
                // The boot CPU saw `Parked` and sent INIT, which succeeded:
                // the core is reset, its TLB empty, and it is `Offline`.
                t.ap[c] = Ap::Abandoned;
                t.tlb[c] = None;
            }
            Ap::Online | Ap::Abandoned => continue,
        }
        out.push(t);
        // Past its online deadline the boot CPU asks an AP that entered to
        // park; it may do so at any point of the AP's bring-up.
        if cfg.may_ask_park && matches!(s.ap[c], Ap::Entered(_)) && !s.park_asked[c] {
            let mut t = *s;
            t.park_asked[c] = true;
            out.push(t);
        }
        if matches!(s.ap[c], Ap::Starting | Ap::Woken) {
            let mut t = *s;
            t.ap[c] = Ap::Abandoned;
            t.tlb[c] = None;
            out.push(t);
        }
    }
}
