// NONOS Operating System (AGPL-3.0-or-later)
//! A CPU dropping the page from its TLB.

use super::config::Config;
use super::state::State;
use super::types::Pge;

/// A CPU drops the page from its TLB. `full` is the full flush, which keeps
/// the GLOBAL entry only if it is a bare CR3 write on a CPU with PGE set.
pub(crate) fn drop_page(s: &mut State, cfg: &Config, c: usize, full: bool) {
    let pge = match cfg.pge {
        Pge::Off => false,
        Pge::BootOnly => c == 0,
        Pge::Every => true,
    };
    if !(full && pge && !cfg.full_toggles_pge) {
        s.tlb[c] = None;
    }
}
