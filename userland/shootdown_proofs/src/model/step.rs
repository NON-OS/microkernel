// NONOS Operating System (AGPL-3.0-or-later)
//! Every state one step away: each actor's moves, in a fixed order.

use super::config::Config;
use super::state::State;

/// Every state one step away from `s`.
pub fn successors(s: &State, cfg: &Config, out: &mut Vec<State>) {
    out.clear();
    super::hardware::step(s, cfg, out);
    super::ap::step(s, cfg, out);
    super::target::step(s, cfg, out);
    super::initiator::step(s, cfg, out);
}
