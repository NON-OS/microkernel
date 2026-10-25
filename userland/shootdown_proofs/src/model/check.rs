// NONOS Operating System (AGPL-3.0-or-later)
//! What a correct round guarantees.

use super::config::Config;
use super::state::{Ap, Handler, Init, State};
use super::types::{Step, Val};

/// What a correct round guarantees, checked in every reachable state.
pub fn check(s: &State, cfg: &Config) -> Result<(), &'static str> {
    for c in 1..cfg.cpus {
        // An AP that parks inside a lock leaves it held for ever, and every
        // later taker spins: the wedge parking exists to avoid. It parks at a
        // checkpoint once asked, so a checkpoint reached while holding a lock
        // with the request set is that wedge.
        if let Ap::Entered(pc) = s.ap[c] {
            let pc = pc as usize;
            if s.park_asked[c]
                && pc < cfg.bring_up.len()
                && cfg.bring_up[pc] == Step::Checkpoint
                && s.holds_lock(c, cfg)
            {
                return Err("an AP can park while holding a lock");
            }
        }
    }
    // Each outstanding ack belongs to a target that will answer it: one that
    // serves its flag now or will once it comes online, or one the waiter
    // answers for. Without that the wait reaches its deadline and stops the
    // machine.
    if matches!(s.init, Init::Waiting) {
        let mut owed = 0u8;
        for c in 1..cfg.cpus {
            let open = s.pending[c] || s.handler[c] != Handler::Idle;
            if !open {
                continue;
            }
            if s.targets & (1 << c) == 0 {
                return Err("a cpu that was not targeted owes an ack");
            }
            if s.ap[c] == Ap::Abandoned && !cfg.ack_for_abandoned {
                return Err("an ack is owed by a cpu that will never answer");
            }
            if s.handler[c] != Handler::Half || cfg.flush_before_ack {
                owed += 1;
            }
        }
        if owed != s.acks {
            return Err("the ack count does not match the open targets");
        }
    }
    // Once the round returns, no CPU that can still run kernel code holds the
    // old translation, and neither can one that has yet to start.
    if s.init == Init::Done {
        for c in 0..cfg.cpus {
            if s.tlb[c] == Some(Val::Old) && s.ap[c] != Ap::Abandoned {
                return Err("a cpu holds the old translation after the round returned");
            }
        }
    }
    Ok(())
}
