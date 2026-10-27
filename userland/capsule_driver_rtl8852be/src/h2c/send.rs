// NONOS Operating System (AGPL-3.0-or-later)

//! Putting a packet on the firmware command queue (`rtw89_h2c_tx` over
//! `rtw89_pci_fwcmd_submit`, Linux v6.12 rtw89 `core.c`, `pci.c`). rtw89
//! frees a command's buffer only once eight more have been taken after it
//! (`rtw89_pci_release_fwcmd`, `RTW89_PCI_MULTITAG`): with eight DMA tags
//! the engine may still be reading it. Here each descriptor owns its
//! buffer, so a packet waits until more than eight descriptors are free.

use super::packet::{fwcmd_wd, h2c_hdr, Cmd, H2C_HDR, WD_BODY};
use super::Fw;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::ring::tx::{TxRing, SLOT};
use crate::wait::until_ms;

/// `RTW89_PCI_MULTITAG`.
pub const MULTITAG: u32 = 8;
/// How long a command waits for the queue to drain far enough.
pub const ROOM_MS: u64 = 500;
/// The largest command body after its H2C header.
pub const BODY_MAX: usize = SLOT - WD_BODY - H2C_HDR;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendFail {
    TooLong,
    /// The engine did not take earlier packets within `ROOM_MS`.
    NoRoom,
}

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

/// Send command `cmd` with `body`: the sequence stamped and stepped, and a
/// receive acknowledgement asked of every fourth, as rtw89 asks.
pub fn h2c<M: DmaMem>(
    r: &impl Mmio,
    ring: &mut TxRing<M>,
    fw: &mut Fw,
    cmd: Cmd,
    body: &[u8],
    dack: bool,
) -> Result<(), SendFail> {
    if body.len() > BODY_MAX {
        return Err(SendFail::TooLong);
    }
    let rack = fw.h2c_seq % 4 == 0;
    let mut pkt = [0u8; SLOT - WD_BODY];
    pkt[..H2C_HDR].copy_from_slice(&h2c_hdr(cmd, fw.h2c_seq, body.len(), rack, dack));
    pkt[H2C_HDR..H2C_HDR + body.len()].copy_from_slice(body);
    fw.h2c_seq = fw.h2c_seq.wrapping_add(1);
    post(r, ring, &pkt[..H2C_HDR + body.len()], false)
}
