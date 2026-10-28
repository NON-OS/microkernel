// NONOS Operating System (AGPL-3.0-or-later)

//! A frame ring with WD pages (`rtw89_pci_txwd_submit`, Linux v6.12 rtw89
//! `pci.c`): each slot holds the WD, its WP info and one address info
//! entry (64 bytes, what the descriptor hands the engine), then the frame
//! the address info points at. The engine reads the frame only when it
//! sends it, after the descriptor is consumed, so a slot is reused only
//! once the frame's release report names it (`rtw89_pci_release_rpp`); the
//! WP sequence is the slot's index.

use super::wd::WD_LEN;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::ring::tx::{TxRing, SLOT};
use crate::ring::wd::{wp_and_addr, ADDR_INFO_SIZE, WP_INFO_SIZE};

/// The WD page the descriptor points at.
pub const PAGE: usize = WD_LEN + WP_INFO_SIZE + ADDR_INFO_SIZE;
/// The longest frame a slot holds.
pub const FRAME_MAX: usize = SLOT - PAGE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Full {
    TooLong,
    /// No descriptor free, or the next slot's frame not yet released.
    Busy,
}

impl<M: DmaMem> TxRing<M> {
    /// Queue `frame` behind `wd` and kick.
    pub fn send_frame(
        &mut self,
        r: &impl Mmio,
        wd: &[u8; WD_LEN],
        frame: &[u8],
    ) -> Result<(), Full> {
        if frame.len() > FRAME_MAX {
            return Err(Full::TooLong);
        }
        let _ = self.reclaim(r);
        let i = self.wp;
        if self.avail() == 0 || self.busy & (1 << i) != 0 {
            return Err(Full::Busy);
        }
        let at = self.slot_off(i);
        let dma = self.mem.bus_at(at + PAGE).ok_or(Full::Busy)?;
        self.mem.write(at, wd);
        self.mem.write(at + WD_LEN, &wp_and_addr(i as u16, frame.len() as u16, dma));
        self.mem.write(at + PAGE, frame);
        if !self.post(PAGE as u16) {
            return Err(Full::Busy);
        }
        self.busy |= 1 << i;
        self.kick(r);
        Ok(())
    }

    /// A release report for slot `seq`; one for no slot is ignored.
    pub fn release(&mut self, seq: u16) {
        if (seq as u32) < self.len {
            self.busy &= !(1 << seq);
        }
    }
}
