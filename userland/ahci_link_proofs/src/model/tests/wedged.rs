// NONOS Operating System (AGPL-3.0-or-later)
//! A port whose command engine will not stop, even after a COMRESET. The
//! HBA may still own the command list and the data buffer, so the driver
//! must not write either, nor set PxCI with the engine off: every request
//! fails at once, answered as no device (out of service until a reboot,
//! never an I/O error worth a retry), and the port is left alone.

use std::time::Duration;

use super::failure::status;
use crate::clock::now;
use crate::engine::flush::flush;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::{CrStuck, Outcome, Rig, Step};
use crate::protocol::E_NODEV;

#[test]
fn a_port_whose_engine_never_stops_is_failed_fast_and_left_alone() {
    let late = Step { ms: 40_000, outcome: Outcome::Clean };
    let mut rig = Rig::new(vec![late], true);
    let regs = rig.regs();
    rig.hba().quirks.cr_stuck = CrStuck::Always;
    assert_eq!(transfer(&mut rig.port, regs, 9, 4, false), Err(AhciError::Timeout));
    let (table, issued) = (rig.table(), rig.hba().count.issued);
    for (lba, write) in [(10, true), (11, false), (12, true)] {
        let t = now();
        let r = transfer(&mut rig.port, regs, lba, 4, write);
        let took = now() - t;
        assert_eq!(r.map_err(|e| status(&rig.port, e)), Err(E_NODEV), "LBA {lba}");
        assert!(took < Duration::from_secs(1), "LBA {lba}: failed after {took:?}");
        assert!(rig.table() == table, "LBA {lba}: command table rewritten under the HBA");
    }
    assert_eq!(rig.port.load(&[0x33; 512]), Err(AhciError::Wedged), "payload copied in");
    assert_eq!(flush(&mut rig.port, regs), Err(AhciError::Wedged));
    assert_eq!(status(&rig.port, AhciError::Wedged), E_NODEV);
    assert!(rig.table() == table, "command table rewritten under the HBA");
    let hba = rig.hba();
    assert_eq!(hba.count.issued, issued, "PxCI set on a port whose engine never stopped");
    assert_eq!(hba.count.issued_stopped, 0, "PxCI set with the engine off");
}

#[test]
fn an_engine_freed_by_comreset_serves_again() {
    let late = Step { ms: 40_000, outcome: Outcome::Clean };
    let mut rig = Rig::new(vec![late], true);
    let regs = rig.regs();
    rig.hba().quirks.cr_stuck = CrStuck::UntilReset;
    assert_eq!(transfer(&mut rig.port, regs, 9, 4, false), Err(AhciError::Timeout));
    assert_eq!(transfer(&mut rig.port, regs, 10, 4, false), Ok(()));
    assert_eq!(rig.hba().count.issued_stopped, 0, "PxCI set with the engine off");
}
