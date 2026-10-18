// NONOS Operating System (AGPL-3.0-or-later)
//! How long a request may hold the port. The kernel stops waiting for the
//! reply after 35 s and reports the request lost, so every path through the
//! driver, recoveries and all, must answer before that (`budget_reply`);
//! and a command the driver issues must be given the time to finish, never
//! dropped at once.

use std::time::Duration;

use crate::clock::now;
use crate::constants::budget::PRE_ISSUE_MS;
use crate::constants::timing::COMMAND_MS;
use crate::engine::transfer::transfer;
use crate::model::{CrStuck, Rig, Step};

/// The least an issued command is given on a served port.
const LEAST: Duration = Duration::from_millis(COMMAND_MS - PRE_ISSUE_MS);

/// A port whose drive is busy for `busy_s` before anything is issued, a
/// COMRESET or not, whose link takes `link_ms` to come back, and whose
/// command engine stops as `cr` says.
pub(super) fn hostile(busy_s: f64, link_ms: u64, cr: CrStuck, script: Vec<Step>) -> Rig {
    let rig = Rig::new(script, true);
    let mut hba = rig.hba();
    hba.quirks.busy_until = Some(now() + Duration::from_secs_f64(busy_s));
    (hba.quirks.link_ms, hba.quirks.cr_stuck) = (link_ms, cr);
    drop(hba);
    rig
}

#[test]
fn a_command_is_given_its_time_or_never_issued() {
    for busy_s in [0.0, 2.5, 30.5, 31.5, 100.0] {
        let mut rig = hostile(busy_s, 10, CrStuck::No, Vec::new());
        let regs = rig.regs();
        let _ = transfer(&mut rig.port, regs, 5, 1, false);
        for given in &rig.hba().count.given {
            assert!(*given >= LEAST, "BSY {busy_s} s: dropped after {given:?}");
        }
    }
}
