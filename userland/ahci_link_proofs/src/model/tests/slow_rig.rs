// NONOS Operating System (AGPL-3.0-or-later)
//! A port whose drive is slow out of the COMRESET a recovery sent, and the
//! requests the tests put to it.

use std::time::Instant;

use crate::constants::ata::{ATA_FLUSH_EXT, ATA_WRITE_DMA_EXT};
use crate::engine::flush::flush;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::{Outcome, Rig, Step};

/// A port whose read timed out, its drive reset for it and `reset_ms` from
/// out of that reset; and when the drive will be ready.
pub(super) fn after_timeout(sclo: bool, reset_ms: u64) -> (Rig, Instant) {
    let mut rig = Rig::new(vec![Step { ms: 40_000, outcome: Outcome::Clean }], sclo);
    rig.hba().quirks.reset_ms = Some(reset_ms);
    let regs = rig.regs();
    assert_eq!(transfer(&mut rig.port, regs, 1_000, 8, false), Err(AhciError::Timeout));
    let ready = rig.hba().at.ready.expect("the drive still in its reset");
    (rig, ready)
}

/// Request `i`: a write of one sector of `i` at LBA 3000 + `i`, or a flush.
/// Gives its answer and the command the drive must have taken for it.
pub(super) fn request(rig: &mut Rig, i: u64) -> (Result<(), AhciError>, (u8, u64)) {
    let regs = rig.regs();
    if i % 2 == 1 {
        return (flush(&mut rig.port, regs), (ATA_FLUSH_EXT, 0));
    }
    rig.load(&[i as u8; 512]);
    (transfer(&mut rig.port, regs, 3_000 + i, 1, true), (ATA_WRITE_DMA_EXT, 3_000 + i))
}
