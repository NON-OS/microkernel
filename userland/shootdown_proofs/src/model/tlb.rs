// NONOS Operating System (AGPL-3.0-or-later)
//! A CPU dropping the page from its TLB.

use super::config::Config;
use super::state::State;

/// A CPU drops the page from its TLB. `full` is a CR3 reload, which keeps a
/// GLOBAL entry when that CPU has CR4.PGE set. APs never set PGE.
pub(crate) fn drop_page(s: &mut State, cfg: &Config, c: usize, full: bool) {
    let pge = c == 0 && cfg.initiator_pge;
    if !(full && pge) {
        s.tlb[c] = None;
    }
}
