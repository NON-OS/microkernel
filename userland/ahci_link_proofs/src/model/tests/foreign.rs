// NONOS Operating System (AGPL-3.0-or-later)
//! A port showing slots the driver never issued before it issues one. The
//! drive may be running a command, so the port is reset, as after issue;
//! slots that outlive that are the HBA's, and the request answers as an I/O
//! error, never with an ATA status that blames the disk.

use super::failure::status;
use crate::engine::transfer::transfer;
use crate::model::Rig;
use crate::protocol::E_IO;

#[test]
fn slots_the_driver_never_issued_are_reset_before_issue() {
    let foreign = [(1u32 << 5, 0u32), (!1, 0), (0, 1), (0, 1 << 31), (1 << 31, u32::MAX)];
    for (ci, sact) in foreign {
        let mut rig = Rig::new(Vec::new(), true);
        let regs = rig.regs();
        (rig.hba().reg.ci, rig.hba().reg.sact) = (ci, sact);
        let r = transfer(&mut rig.port, regs, 9, 1, false);
        let at = format!("PxCI {ci:#x} PxSACT {sact:#x}");
        assert_eq!((r, rig.hba().count.comresets), (Ok(()), 1), "{at}");
        assert_eq!(rig.hba().count.issued_stopped, 0, "{at}: issued with the engine off");
    }
}

#[test]
fn slots_that_outlive_recovery_are_an_io_error_not_the_disks() {
    for (ci, sact) in [(1u32 << 5, 0u32), (!1, 0), (0, 1), (1 << 31, u32::MAX)] {
        let mut rig = Rig::new(Vec::new(), true);
        let regs = rig.regs();
        rig.hba().quirks.sticky = (ci, sact);
        let r = transfer(&mut rig.port, regs, 9, 1, false);
        let at = format!("PxCI {ci:#x} PxSACT {sact:#x}");
        assert_eq!(r.map_err(|e| status(&rig.port, e)), Err(E_IO), "{at}");
        assert_eq!(rig.hba().count.issued, 0, "{at}: issued over slots not the driver's");
    }
}
