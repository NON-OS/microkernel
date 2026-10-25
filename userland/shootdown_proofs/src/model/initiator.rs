// NONOS Operating System (AGPL-3.0-or-later)
//! The initiator: write, local flush, select, mark and send.

use super::config::Config;
use super::state::{Ap, Init, State};
use super::tlb::drop_page;
use super::types::{Flush, Val};

pub(super) fn step(s: &State, cfg: &Config, out: &mut Vec<State>) {
    let n = cfg.cpus;
    // The initiator.
    let mut t = *s;
    let full = cfg.flush == Flush::Full;
    match s.init {
        Init::Start => {
            if cfg.write_before_select {
                t.pte = Val::New;
                t.init = Init::Written;
            } else {
                t.init = Init::Selecting(1);
            }
        }
        Init::Written => {
            drop_page(&mut t, cfg, 0, full);
            t.init = if cfg.write_before_select { Init::LocalFlushed } else { Init::Sending(1) };
        }
        Init::LocalFlushed => t.init = Init::Selecting(1),
        Init::Selecting(i) => {
            let c = i as usize;
            if c < n {
                let target =
                    s.ap[c] == Ap::Online || (cfg.target_starting && s.descriptor_starting(c, cfg));
                if target {
                    t.targets |= 1 << c;
                }
                t.init = Init::Selecting(i + 1);
            } else if cfg.write_before_select {
                t.init = Init::Sending(1);
            } else {
                // The mutant: targets chosen, the write only now.
                t.pte = Val::New;
                t.init = Init::Written;
            }
        }
        Init::Sending(i) => {
            let c = i as usize;
            if c < n {
                if s.targets & (1 << c) != 0 {
                    t.acks += 1;
                    t.pending[c] = true;
                }
                t.init = Init::Sending(i + 1);
            } else {
                t.init = Init::Waiting;
            }
        }
        Init::Waiting => return super::waiter::step(s, cfg, out),
        Init::Done => return,
    }
    out.push(t);
}
