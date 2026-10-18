// NONOS Operating System (AGPL-3.0-or-later)
//! The wait before issue on a port with no device or the PHY offline (the
//! disk pulled): the request is refused at once, never issued to and
//! waited on for a command's budget, nor reset. A device without a link is
//! `unlinked`; slots the driver never issued are `foreign`.

use std::time::Duration;

use crate::clock::now;
use crate::engine::flush::flush;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::state::LINK_UP;
use crate::model::Rig;

#[test]
fn a_pulled_disk_is_refused_at_once_and_served_once_back() {
    let mut rig = Rig::new(Vec::new(), true);
    let regs = rig.regs();
    assert_eq!(transfer(&mut rig.port, regs, 1, 1, false), Ok(()));
    rig.hba().quirks.stuck_det = Some(0);
    let issued = rig.hba().count.issued;
    // Every DET but 1 and 3, under any other PxSSTS bits.
    for ssts in [0u32, 0x100, 0x120, 0xffff_fff0] {
        for det in [0u32, 2, 4, 5, 7, 8, 0xf] {
            rig.hba().reg.ssts = ssts | det;
            for request in 0..3 {
                let t = now();
                let r = match request {
                    0 => transfer(&mut rig.port, regs, 2, 1, false),
                    1 => transfer(&mut rig.port, regs, 2, 1, true),
                    _ => flush(&mut rig.port, regs),
                };
                let at = format!("PxSSTS {:#x}, request {request}", ssts | det);
                assert_eq!(r, Err(AhciError::NoDisk), "{at}");
                assert!(now() - t < Duration::from_millis(50), "{at}: answered late");
            }
        }
    }
    let hba = rig.hba();
    let after = (hba.count.issued, hba.count.comresets);
    assert_eq!(after, (issued, 0), "issued to, or reset, with no link");
    drop(hba);
    (rig.hba().quirks.stuck_det, rig.hba().reg.ssts) = (None, LINK_UP);
    assert_eq!(transfer(&mut rig.port, regs, 3, 1, false), Ok(()), "not served once back");
}
