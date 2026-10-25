// NONOS Operating System (AGPL-3.0-or-later)
//! Targets: from the pending flag, through the flush, to the ack.

use super::config::Config;
use super::state::{Handler, State};
use super::tlb::drop_page;
use super::types::Flush;

pub(super) fn step(s: &State, cfg: &Config, out: &mut Vec<State>) {
    let n = cfg.cpus;
    // Targets.
    for c in 1..n {
        let mut t = *s;
        let full = cfg.flush == Flush::Full;
        match s.handler[c] {
            Handler::Idle if s.pending[c] && (s.interruptible(c) || s.serves(c)) => {
                t.pending[c] = false;
                t.handler[c] = Handler::Taken;
            }
            Handler::Taken => {
                if cfg.flush_before_ack {
                    drop_page(&mut t, cfg, c, full);
                } else {
                    t.acks -= 1;
                }
                t.handler[c] = Handler::Half;
            }
            Handler::Half => {
                if cfg.flush_before_ack {
                    t.acks -= 1;
                } else {
                    drop_page(&mut t, cfg, c, full);
                }
                t.handler[c] = Handler::Idle;
            }
            _ => continue,
        }
        out.push(t);
    }
}
