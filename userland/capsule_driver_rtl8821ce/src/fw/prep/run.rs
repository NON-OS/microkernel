// NONOS Operating System (AGPL-3.0-or-later)

//! The download's prologue and close-out.

use super::super::regs::{
    BTI_PAGE_OVF, CHECK_SUM_OK, FW_DW_RDY, FW_READY, FW_READY_MASK, MCUFWDL_EN, REG_MCUFW_CTRL,
    REG_TXDMA_STATUS,
};
use super::backup::{reg_backup, reg_restore};
use super::cpu::{halt_cpu, reset_platform, start_cpu};
use super::kinds::{Backup, Finish};
use crate::regs::Mmio;

/// Reads allowed before the firmware-ready handshake is declared failed. The
/// 8051 needs real time to boot and raise the ready bits, so this is generous.
const READY_POLL_LIMIT: u32 = 8_000_000;

/// Prepare for download: stop the 8051, map the queues so the beacon queue can
/// stage reserved pages, and reset the WL platform. Returns the saved registers.
pub fn begin<M: Mmio>(mmio: &M) -> Backup {
    halt_cpu(mmio);
    let backup = reg_backup(mmio);
    reset_platform(mmio);
    backup
}

/// Finish download: restore the queues, clear the download state and release the
/// 8051, then wait for it to report ready. Distinguishes a bad-checksum close-out
/// (the DDMA delivered wrong bytes) from a boot that never reaches ready, and
/// carries the control register in both so the failure is legible without serial.
pub fn finish<M: Mmio>(mmio: &M, backup: Backup) -> Finish {
    reg_restore(mmio, backup);

    // download_firmware_end_flow: clear the page-overflow status, then only mark
    // the firmware downloaded if the hardware confirmed both section checksums.
    mmio.write32(REG_TXDMA_STATUS, BTI_PAGE_OVF);
    let ctrl = mmio.read16(REG_MCUFW_CTRL);
    if ctrl & CHECK_SUM_OK != CHECK_SUM_OK {
        return Finish::ChecksumBad(ctrl);
    }
    mmio.write16(REG_MCUFW_CTRL, (ctrl | FW_DW_RDY) & !MCUFWDL_EN);

    start_cpu(mmio);

    let mut last = 0u16;
    for _ in 0..READY_POLL_LIMIT {
        last = mmio.read16(REG_MCUFW_CTRL);
        if last & FW_READY_MASK == FW_READY {
            return Finish::Ready;
        }
        core::hint::spin_loop();
    }
    Finish::NotReady(last)
}
