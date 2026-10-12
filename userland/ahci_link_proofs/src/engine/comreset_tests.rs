// NONOS Operating System (AGPL-3.0-or-later)
//! After the COMRESET that ends an abandoned command, the wait for the link
//! to come back (PxSSTS.DET 3) goes by the clock, not by a count of looks:
//! a link that takes longer is waited for, one that never comes is given
//! up on inside LINK_TIMEOUT_MS. And the engine starts again with nothing
//! the reset latched left in PxSERR or PxIS.

use std::time::Duration;

use super::cause::Cause;
use super::recover::recover;
use super::recover_cause_tests::recovered;
use crate::error::AhciError;
use crate::model::{CrStuck, Rig};

#[test]
fn the_link_is_waited_for_by_the_clock() {
    for link_ms in [1, 50, 500, 1_900] {
        let (r, resets, _, took) = recovered(0x50, true, Cause::Abandoned, link_ms);
        assert_eq!((r, resets), (Ok(()), 1), "link back after {link_ms} ms");
        assert!(took >= Duration::from_millis(link_ms), "{link_ms} ms link, done in {took:?}");
    }
    let (r, _, _, took) = recovered(0x50, true, Cause::Abandoned, 60_000);
    assert_eq!(r, Err(AhciError::Timeout), "a link that never comes back");
    assert!(took < Duration::from_secs(5), "gave up after {took:?}");
}

#[test]
fn the_engine_starts_again_with_nothing_latched() {
    for cr in [CrStuck::No, CrStuck::UntilReset] {
        for tfd in [0x50, 0x51, 0x80, 0x58] {
            let mut rig = Rig::new(Vec::new(), true);
            (rig.hba().reg.tfd, rig.hba().quirks.cr_stuck) = (tfd, cr);
            let regs = rig.regs();
            assert_eq!(recover(&mut rig.port, regs, Cause::Abandoned), Ok(()), "PxTFD {tfd:#x}");
            let hba = rig.hba();
            assert_eq!(hba.count.comresets, 1, "PxTFD {tfd:#x}");
            let at = format!("PxTFD {tfd:#x}: PxSERR and PxIS as ST was set");
            assert_eq!(hba.count.started_with, [(0, 0)], "{at}");
        }
    }
}
