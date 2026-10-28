// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_mac_partial_init` (Linux v6.12 rtw89 `mac.c`) for the 8852B,
//! which has no BB MCU: power on (retried once from off), HCI DMA on, the
//! DMAC pre-init, the PCIe pre-init with the rings programmed, and the
//! firmware downloaded.

use super::Stop;
use crate::dma::DmaMem;
use crate::fw::{self, header::Header};
use crate::h2c::Fw;
use crate::mac::pre::{dmac_pre_init, hci_dma_on};
use crate::pcie;
use crate::pwr::{self, Board};
use crate::regs::Mmio;
use crate::ring::Rings;

pub fn partial_init<M: DmaMem>(
    r: &impl Mmio,
    rings: &mut Rings<M>,
    fw: &mut Fw,
    board: &Board,
    suit: &[u8],
    hdr: &Header,
) -> Result<(), Stop> {
    pwr::power_up(r, board).map_err(Stop::Power)?;
    hci_dma_on(r);
    dmac_pre_init(r).map_err(Stop::Mac)?;
    pcie::phy_setup(r).map_err(Stop::Pcie)?;
    pcie::dma_setup(r, rings).map_err(Stop::Pcie)?;
    fw::download(r, &mut rings.ch12, fw, suit, hdr).map_err(Stop::Firmware)
}
