// NONOS Operating System (AGPL-3.0-or-later)
//! The initiator's wait: done once every ack is in, answering meanwhile for
//! targets the boot CPU gave up on.

use super::config::Config;
use super::state::{Init, State};

pub(super) fn step(s: &State, cfg: &Config, out: &mut Vec<State>) {
    let n = cfg.cpus;
    if s.acks == 0 {
        let mut t = *s;
        t.init = Init::Done;
        out.push(t);
        return;
    }
    // The waiter's own pass over targets the boot CPU gave up on: it
    // takes the pending flag, so exactly one side acknowledges.
    if cfg.ack_for_abandoned {
        for c in 1..n {
            if s.pending[c] && s.descriptor_offline(c, cfg) {
                let mut t = *s;
                t.pending[c] = false;
                t.acks -= 1;
                out.push(t);
            }
        }
    }
}
