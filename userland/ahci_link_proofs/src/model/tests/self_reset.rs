// NONOS Operating System (AGPL-3.0-or-later)
//! A drive that resets itself while it holds a flush or a write. It drops
//! the command and posts its signature once out of the reset, and the HBA
//! takes that signature as the end of slot 0, so nothing but the link
//! change it latched (PxIS.PRCS and PCS) says the command never ran. A
//! flush is the durability barrier: answered Ok, it would promise data on
//! the medium that is not. The request fails, the drive is reset, and the
//! next is served. A change latched before a request is not its own.

use crate::clock::mk_idle_ms;
use crate::constants::ata::ATA_FLUSH_EXT;
use crate::engine::flush::flush;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::state::{DIAG_N, DIAG_X};
use crate::model::{Outcome, Rig, Step};

#[test]
fn a_command_the_drive_dropped_as_it_reset_itself_fails() {
    let mut wrong = Vec::new();
    for reset_ms in [None, Some(500), Some(5_000)] {
        for write in [false, true] {
            let at = format!("drive {reset_ms:?} ms out of its reset, write {write}");
            let mut rig = Rig::new(vec![Step { ms: 5_000, outcome: Outcome::Clean }], true);
            (rig.hba().quirks.reset_ms, rig.hba().quirks.self_reset_ms) = (reset_ms, Some(100));
            let regs = rig.regs();
            let r = if write {
                rig.load(&[0x77; 512]);
                transfer(&mut rig.port, regs, 900, 1, true)
            } else {
                flush(&mut rig.port, regs)
            };
            let resets = rig.hba().count.comresets;
            if (r, resets) != (Err(AhciError::HbaFailed), 1) {
                wrong.push(format!("{at}: answered {r:?} after {resets} COMRESETs"));
                continue;
            }
            let _ = mk_idle_ms(reset_ms.unwrap_or(0));
            assert_eq!(flush(&mut rig.port, regs), Ok(()), "{at}: the next request");
            let last = rig.hba().drive.taken.last().map(|c| c.op);
            assert_eq!(last, Some(ATA_FLUSH_EXT), "{at}: the next flush never ran");
        }
    }
    assert!(wrong.is_empty(), "not failed and reset: {wrong:#?}");
}

#[test]
fn a_link_change_latched_before_the_request_is_not_its_own() {
    for serr in [DIAG_N, DIAG_X, DIAG_N | DIAG_X, u32::MAX] {
        let mut rig = Rig::new(Vec::new(), true);
        let regs = rig.regs();
        rig.hba().reg.serr = serr;
        assert_eq!(flush(&mut rig.port, regs), Ok(()), "PxSERR {serr:#x}");
        assert_eq!(rig.hba().count.comresets, 0, "PxSERR {serr:#x}");
    }
}
