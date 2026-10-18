// NONOS Operating System (AGPL-3.0-or-later)
//! A device whose link is down (PxSSTS.DET 1). Its link is reset, but at
//! most once in ATA's 31 s window: inside it a request is refused at once
//! as an I/O error and the COMRESET sent is left to work; after it, one
//! more is sent; and a link back at DET 3 is served.

use std::time::Duration;

use super::failure::status;
use crate::clock::{mk_idle_ms, now};
use crate::constants::budget::PRE_ISSUE_MS;
use crate::constants::timing::{READY_MS, RESET_WINDOW_MS};
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::state::LINK_UP;
use crate::model::Rig;
use crate::protocol::E_IO;

#[test]
fn a_device_without_a_link_is_reset_and_served() {
    for ssts in [0x1u32, 0x121, 0xffff_fff1] {
        let mut rig = Rig::new(Vec::new(), true);
        let regs = rig.regs();
        rig.hba().reg.ssts = ssts;
        assert_eq!(transfer(&mut rig.port, regs, 4, 1, false), Ok(()), "PxSSTS {ssts:#x}");
        assert_eq!(rig.hba().count.comresets, 1, "PxSSTS {ssts:#x}");
    }
}

#[test]
fn a_link_that_stays_down_is_reset_once_a_window() {
    let (ready, window) = (Duration::from_millis(READY_MS), Duration::from_millis(RESET_WINDOW_MS));
    let mut rig = Rig::new(Vec::new(), true);
    let regs = rig.regs();
    (rig.hba().reg.ssts, rig.hba().quirks.stuck_det) = (0x121, Some(1));
    let t = now();
    assert!(transfer(&mut rig.port, regs, 4, 1, false).is_err());
    assert!(now() - t <= Duration::from_millis(PRE_ISSUE_MS), "the reset took {:?}", now() - t);
    let first = rig.hba().at.reset_from.expect("the link reset");
    let mut asked = 0;
    while rig.hba().count.comresets == 1 {
        assert!(asked < 40, "never reset again");
        let t = now();
        let r = transfer(&mut rig.port, regs, 4, 1, false);
        if rig.hba().count.comresets == 1 {
            assert_eq!(r.map_err(|e| (e, status(&rig.port, e))), Err((AhciError::NoDisk, E_IO)));
            assert!(now() - t <= ready, "request {asked} held {:?}", now() - t);
        }
        asked += 1;
        let _ = mk_idle_ms(READY_MS);
    }
    let again = rig.hba().at.reset_from.expect("the link reset") - first;
    // The driver opens its window a moment before the model sees DET 1.
    assert!(again + Duration::from_millis(1) >= window, "reset again {again:?} after");
    assert!(again <= window + ready + ready, "reset again only {again:?} after");
    // The PHY comes back by itself, the device's signature posted: served,
    // no COMRESET more.
    let mut hba = rig.hba();
    (hba.quirks.stuck_det, hba.reg.ssts, hba.reg.tfd) = (None, LINK_UP, 0x50);
    drop(hba);
    assert_eq!(transfer(&mut rig.port, regs, 4, 1, false), Ok(()));
    assert_eq!(rig.hba().count.comresets, 2);
}
