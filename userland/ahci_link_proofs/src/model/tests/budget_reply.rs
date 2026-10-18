// NONOS Operating System (AGPL-3.0-or-later)
//! Every path through a request, waits and recoveries all, answers before
//! the kernel stops waiting for the reply (35 s), on a port held up in each
//! way the driver knows before issue, with a command that never ends.

use std::time::Duration;

use super::budget::hostile;
use crate::clock::now;
use crate::constants::budget::KERNEL_REPLY_MS;
use crate::engine::transfer::transfer;
use crate::model::state::LINK_UP;
use crate::model::{CrStuck, Outcome, Step};

const KERNEL_WAIT: Duration = Duration::from_millis(KERNEL_REPLY_MS);

#[test]
fn every_request_answers_inside_the_kernels_wait() {
    let never = vec![Step { ms: 1_000_000, outcome: Outcome::Clean }];
    /*
     * Busy for 30.5 s, a wait before issue as long as the command's budget
     * would issue it at 30.5 s and then wait on it: past the kernel's wait.
     * The link down (DET 1), or slots the driver never issued: a COMRESET
     * before issue, the drive 0.99 s out of it, then a command that never
     * ends and the same again after it. BSY s, CR, PxSSTS, PxCI, PxSACT:
     */
    let cases = [
        (100.0, CrStuck::No, LINK_UP, 0, 0),
        (30.5, CrStuck::No, LINK_UP, 0, 0),
        (0.0, CrStuck::UntilReset, LINK_UP, 0, 0),
        (100.0, CrStuck::Always, LINK_UP, 0, 0),
        (0.0, CrStuck::No, 0x121, 0, 0),
        (0.0, CrStuck::No, LINK_UP, 1 << 5, 0),
        (0.0, CrStuck::No, LINK_UP, 0, 1),
    ];
    let mut late = Vec::new();
    for (busy_s, cr, ssts, ci, sact) in cases {
        let mut rig = hostile(busy_s, 1_990, cr, never.clone());
        let reset_ms = (ssts != LINK_UP || ci | sact != 0).then_some(990);
        let mut hba = rig.hba();
        (hba.reg.ssts, hba.reg.ci, hba.reg.sact, hba.quirks.reset_ms) = (ssts, ci, sact, reset_ms);
        drop(hba);
        let (regs, t) = (rig.regs(), now());
        let r = transfer(&mut rig.port, regs, 5, 1, false);
        let (took, at) = (now() - t, format!("BSY {busy_s} s, {ssts:#x} {ci:#x} {sact:#x}"));
        if took >= KERNEL_WAIT {
            late.push(format!("{at}: answered {r:?} after {took:?}"));
        }
    }
    assert!(late.is_empty(), "past the kernel's wait: {late:#?}");
}
