// NONOS Operating System (AGPL-3.0-or-later)

//! A section's chunks through DDMA with its checksum.

use crate::regs::Mmio;

use super::{
    DDMA_CHKSUM_STS, DMEM_CHKSUM_OK, DMEM_DW_OK, IMEM_CHKSUM_OK, IMEM_DW_OK, OCPBASE_DMEM,
    REG_DDMA_CH0CTRL, REG_MCUFW_CTRL,
};

/// Record a finished section in the firmware-download control register and
/// report whether its DDMA checksum validated. rtw88 `check_fw_checksum` does
/// this in software after the section's last chunk: the hardware never sets the
/// download-ok and checksum-ok bits itself, so the driver reads the DDMA
/// checksum status and writes the matching pair of bits for the section's memory
/// (IMEM below `OCPBASE_DMEM`, DMEM at or above it). Without this the control
/// register keeps only the download-enable bit and the close-out reads the
/// firmware as never checksummed. `dst` is the section's on-chip base address.
pub fn record_section<M: Mmio>(mmio: &M, dst: u32) -> bool {
    let ok = mmio.read32(REG_DDMA_CH0CTRL) & DDMA_CHKSUM_STS == 0;
    // The download-ok and checksum-ok bits all live in the low byte; rtw88
    // touches only that byte so the reserved high bits the card owns are kept.
    let ctrl = mmio.read8(REG_MCUFW_CTRL);
    let (dw_ok, chksum_ok) = if dst < OCPBASE_DMEM {
        (IMEM_DW_OK as u8, IMEM_CHKSUM_OK as u8)
    } else {
        (DMEM_DW_OK as u8, DMEM_CHKSUM_OK as u8)
    };
    // A validated section marks both download-ok and checksum-ok; a failed one
    // marks download-ok but leaves checksum-ok clear, matching rtw88.
    let ctrl = if ok { ctrl | dw_ok | chksum_ok } else { (ctrl | dw_ok) & !chksum_ok };
    mmio.write8(REG_MCUFW_CTRL, ctrl);
    ok
}
