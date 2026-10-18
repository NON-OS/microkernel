// NONOS Operating System (AGPL-3.0-or-later)
//! The COMRESET that frees a command engine whose CR will not clear resets
//! the drive as any other does, so it opens ATA's window too: a drive slow
//! out of it is waited for, not reset again, and served once ready.

use std::time::Duration;

use crate::clock::{mk_idle_ms, now};
use crate::constants::timing::READY_MS;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::{CrStuck, Outcome, Rig, Step};

#[test]
fn the_comreset_that_frees_a_stuck_engine_opens_the_window() {
    let ready_ms = Duration::from_millis(READY_MS);
    for sclo in [true, false] {
        let mut rig = Rig::new(vec![Step { ms: 40_000, outcome: Outcome::Clean }], sclo);
        let regs = rig.regs();
        (rig.hba().quirks.cr_stuck, rig.hba().quirks.reset_ms) = (CrStuck::UntilReset, Some(5_000));
        assert_eq!(transfer(&mut rig.port, regs, 1_000, 8, false), Err(AhciError::Timeout));
        assert_eq!(rig.hba().count.comresets, 1, "SCLO {sclo}");
        let ready = rig.hba().at.ready.expect("the drive still in its reset");
        while now() + ready_ms < ready {
            let t = now();
            assert_eq!(transfer(&mut rig.port, regs, 2_000, 8, false), Err(AhciError::Timeout));
            assert!(now() - t <= ready_ms + Duration::from_millis(100), "SCLO {sclo}: held");
            assert_eq!(rig.hba().count.comresets, 1, "SCLO {sclo}: reset inside the window");
        }
        let _ = mk_idle_ms(READY_MS);
        assert_eq!(transfer(&mut rig.port, regs, 2_000, 8, false), Ok(()), "SCLO {sclo}");
        assert_eq!(rig.hba().count.comresets, 1, "SCLO {sclo}");
    }
}
