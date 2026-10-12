// NONOS Operating System (AGPL-3.0-or-later)
//! FLUSH CACHE EXT through the command path, and one the drive is still
//! running when the driver gives up on it: the drive is reset before the
//! next command, so the flush's late end completes nothing else.

use crate::constants::ata::{ATA_FLUSH_EXT, SECTOR_SIZE};
use crate::engine::flush::flush;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::{pattern, Outcome, Rig, Step};

#[test]
fn a_flush_is_issued_and_done() {
    let mut rig = Rig::new(Vec::new(), true);
    let regs = rig.regs();
    assert_eq!(flush(&mut rig.port, regs), Ok(()));
    assert_eq!(rig.hba().drive.taken.last().map(|c| c.op), Some(ATA_FLUSH_EXT));
}

#[test]
fn a_flush_the_drive_never_ends_is_reset_before_the_next_command() {
    for sclo in [true, false] {
        let late = Step { ms: 40_000, outcome: Outcome::Clean };
        let mut rig = Rig::new(vec![late], sclo);
        let regs = rig.regs();
        assert_eq!(flush(&mut rig.port, regs), Err(AhciError::Timeout));
        assert_eq!(rig.hba().count.comresets, 1, "SCLO {sclo}");
        assert_eq!(transfer(&mut rig.port, regs, 64, 2, false), Ok(()));
        let want: Vec<u8> = (64..66).flat_map(pattern).collect();
        assert_eq!(rig.data(2 * SECTOR_SIZE), want, "SCLO {sclo}");
    }
}
