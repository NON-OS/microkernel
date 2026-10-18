// NONOS Operating System (AGPL-3.0-or-later)
//! A command the HBA fails with an error of its own (interface, host bus,
//! overflow) while the drive goes on with it. The drive has posted no
//! status, so on an HBA that leaves PxTFD as it was (QEMU) nothing there
//! says it is busy: it must be reset all the same, or its late data lands
//! in the next request.

use crate::constants::ata::SECTOR_SIZE;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::{pattern, Outcome, Rig, Step};

/// IFS, HBDS, HBFS, OFS, and IFS with TFES.
const FATAL: [u32; 5] = [1 << 27, 1 << 28, 1 << 29, 1 << 24, 1 << 27 | 1 << 30];

#[test]
fn a_command_the_hba_failed_is_ended_before_the_next_one() {
    for bits in FATAL {
        for ms in [5, 40, 1_000] {
            let mut rig = Rig::new(vec![Step { ms, outcome: Outcome::Clean }], true);
            let regs = rig.regs();
            rig.hba().quirks.fatal_on_send = Some(bits);
            let r = transfer(&mut rig.port, regs, 1_000, 8, false);
            assert_eq!(r, Err(AhciError::HbaFailed), "PxIS {bits:#x}");
            assert_eq!(transfer(&mut rig.port, regs, 2_000, 8, false), Ok(()));
            let want: Vec<u8> = (2_000..2_008).flat_map(pattern).collect();
            let late = (1_000..1_008).flat_map(pattern).collect::<Vec<u8>>();
            let got = rig.data(8 * SECTOR_SIZE);
            assert!(got != late, "PxIS {bits:#x}, drive done after {ms} ms: the late data");
            assert!(got == want, "PxIS {bits:#x}, drive done after {ms} ms: wrong data");
            assert_eq!(rig.hba().count.comresets, 1, "PxIS {bits:#x}: drive not reset");
        }
    }
}

#[test]
fn an_error_the_drive_posted_needs_no_reset() {
    let unc = Step { ms: 1, outcome: Outcome::Error(0x40) };
    let mut rig = Rig::new(vec![unc], true);
    let regs = rig.regs();
    let r = transfer(&mut rig.port, regs, 1_000, 8, false);
    assert_eq!(r, Err(AhciError::CommandFailed));
    assert_eq!(rig.hba().count.comresets, 0, "a drive that ended its command was reset");
}
