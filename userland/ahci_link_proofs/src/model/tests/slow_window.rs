// NONOS Operating System (AGPL-3.0-or-later)
//! A drive that never comes out of the COMRESET a recovery sent. Inside
//! ATA's 31 s each request waits READY_MS for it and fails; once that
//! window has passed, the drive is reset again, and every request still
//! answers inside the kernel's wait.

use std::time::Duration;

use super::slow_rig::after_timeout;
use crate::clock::now;
use crate::constants::budget::KERNEL_REPLY_MS;
use crate::constants::timing::{READY_MS, RESET_WINDOW_MS};
use crate::engine::transfer::transfer;
use crate::error::AhciError;

#[test]
fn a_drive_still_busy_past_atas_window_is_reset_again() {
    let (window, ready) = (Duration::from_millis(RESET_WINDOW_MS), Duration::from_millis(READY_MS));
    for sclo in [true, false] {
        let (mut rig, _) = after_timeout(sclo, 100_000);
        let regs = rig.regs();
        let first = rig.hba().at.reset_from.expect("the drive reset");
        let mut asked = 0;
        while rig.hba().count.comresets == 1 {
            assert!(asked < 40, "SCLO {sclo}: never reset again");
            let t = now();
            assert_eq!(transfer(&mut rig.port, regs, 7, 1, false), Err(AhciError::Timeout));
            let took = now() - t;
            assert!(took < Duration::from_millis(KERNEL_REPLY_MS), "SCLO {sclo}: {took:?}");
            if rig.hba().count.comresets == 1 {
                assert!(took <= ready + Duration::from_millis(100), "SCLO {sclo}: held {took:?}");
            }
            asked += 1;
        }
        // The driver opens its window a moment before the model sees DET 1.
        let hba = rig.hba();
        let again = hba.at.reset_from.expect("the drive reset") - first;
        let early = again + Duration::from_millis(1) < window;
        assert!(!early, "SCLO {sclo}: reset again {again:?} after the first");
        assert!(again <= window + ready + ready, "SCLO {sclo}: reset again only {again:?} after");
        assert_eq!((hba.count.comresets, hba.count.clo, hba.count.lost), (2, 0, 0), "SCLO {sclo}");
    }
}
