// NONOS Operating System (AGPL-3.0-or-later)
//! The hardware: any CPU running kernel code may fill or evict.

use super::config::Config;
use super::state::State;

pub(super) fn step(s: &State, cfg: &Config, out: &mut Vec<State>) {
    let n = cfg.cpus;
    // The hardware: any CPU running kernel code may fill or evict.
    for c in 0..n {
        if s.runs_kernel(c) {
            let mut t = *s;
            t.tlb[c] = match s.tlb[c] {
                None => Some(s.pte),
                Some(_) => None,
            };
            out.push(t);
        }
    }
}
