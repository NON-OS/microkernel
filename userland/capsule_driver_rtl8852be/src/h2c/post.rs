// NONOS Operating System (AGPL-3.0-or-later)

//! Putting a packet on the firmware command queue (`rtw89_h2c_tx` over
//! `rtw89_pci_fwcmd_submit`, Linux v6.12 rtw89 `core.c`, `pci.c`). rtw89
//! frees a command's buffer only once eight more have been taken after it
//! (`rtw89_pci_release_fwcmd`, `RTW89_PCI_MULTITAG`): with eight DMA tags
//! the engine may still be reading it. Here each descriptor owns its
//! buffer, so a packet waits until more than eight descriptors are free.

use super::packet::{fwcmd_wd, WD_BODY};
use super::send::SendFail;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::ring::tx::{TxRing, SLOT};
use crate::wait::until_ms;

/// `RTW89_PCI_MULTITAG`.
pub const MULTITAG: u32 = 8;
/// How long a command waits for the queue to drain far enough.
pub const ROOM_MS: u64 = 500;

/// Queue `pkt` (an H2C header and body, or a download chunk) and kick.
pub fn post<M: DmaMem>(
    r: &impl Mmio,
    ring: &mut TxRing<M>,
    pkt: &[u8],
    fw_dl: bool,
) -> Result<(), SendFail> {
    if pkt.len() > SLOT - WD_BODY {
        return Err(SendFail::TooLong);
    }
    if !until_ms(ROOM_MS, || ring.reclaim(r) && ring.avail() > MULTITAG) {
        return Err(SendFail::NoRoom);
    }
    let at = ring.slot_off(ring.wp);
    ring.mem.write(at, &fwcmd_wd(pkt, fw_dl));
    ring.mem.write(at + WD_BODY, pkt);
    if !ring.post((WD_BODY + pkt.len()) as u16) {
        return Err(SendFail::NoRoom);
    }
    ring.kick(r);
    Ok(())
}
