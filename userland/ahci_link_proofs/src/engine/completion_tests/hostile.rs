// NONOS Operating System (AGPL-3.0-or-later)
//! A hostile port: whatever PxCI, PxSACT, PxIS and PxTFD read, and whenever
//! they change, the wait ends in Done only on a clean status read after a
//! clear slot bit. Anything else is an error, never a served sector. This
//! file restates the AHCI bits apart from the driver's source.

use super::model_port::Snap;
use crate::error::AhciError;

pub(super) const FATAL_IS: u32 = 0x7800_0000;
pub(super) const OFS: u32 = 1 << 24;
/// PRCS and PCS: PhyRdy changed, or the device sent COMINIT.
pub(super) const LINK: u32 = 1 << 22 | 1 << 6;

pub(super) fn rounds(full: u64) -> u64 {
    if cfg!(miri) {
        300
    } else {
        full
    }
}

pub(super) fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

pub(super) fn seeded(seed: u64) -> u64 {
    seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1
}

/// How the wait must end on a port that shows `s` from some read onward.
/// PxIS errors and slots the driver never issued fail the command at once;
/// ERR in PxTFD fails it only once slot 0 is clear, since until then PxTFD
/// may hold the previous command's status (AHCI 1.3.1, 3.3.8 and 5.6.2).
pub(super) fn expect(s: Snap) -> Result<(), AhciError> {
    let slot_clear = s.ci & 1 == 0;
    let foreign = s.ci & !1 != 0 || s.sact != 0;
    let failed = s.is & (FATAL_IS | OFS | LINK) != 0 || foreign || slot_clear && s.tfd & 0x01 != 0;
    if failed {
        return Err(AhciError::CommandFailed);
    }
    if !slot_clear || s.tfd & 0x88 != 0 {
        return Err(AhciError::Timeout);
    }
    Ok(())
}

/// One register value, clean three times in four so every outcome is
/// reached, anything at all otherwise.
pub(super) fn draw(s: &mut u64, clean: u32) -> u32 {
    let r = xorshift(s);
    match r & 7 {
        0 | 1 => xorshift(s) as u32,
        2 => clean | (1 << ((r >> 8) % 32)),
        _ => clean,
    }
}

pub(super) fn hostile_snap(s: &mut u64) -> Snap {
    Snap { ci: draw(s, 0), sact: draw(s, 0), is: draw(s, 0x1), tfd: draw(s, 0x50) }
}
