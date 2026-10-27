// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_mac_init` (Linux v6.12 rtw89 `mac.c`): the partial init again
//! (now with the board's efuse facts for the power sequence), BB and RF
//! enabled, the system and TRX init, the PCIe post-init (LTR, address
//! mode, every TX DMA channel let run), and the offload configuration
//! command. On failure rtw89 powers the MAC off; so does this.

use super::probe::{board_of, Probe};
use super::{partial_init, Stop};
use crate::dma::DmaMem;
use crate::fw;
use crate::h2c::{init_cmds, Fw};
use crate::mac::{sys, trx};
use crate::pcie;
use crate::pwr;
use crate::regs::Mmio;
use crate::ring::Rings;

pub fn mac_init<M: DmaMem>(
    r: &impl Mmio,
    rings: &mut Rings<M>,
    fw: &mut Fw,
    p: &Probe,
) -> Result<(), Stop> {
    let board = board_of(p.cv, &p.efuse);
    let run = |rings: &mut Rings<M>, fw: &mut Fw| -> Result<(), Stop> {
        let (suit, hdr) = fw::prepare(fw::IMAGE, p.cv).map_err(Stop::Firmware)?;
        partial_init(r, rings, fw, &board, suit, &hdr)?;
        sys::enable_bb_rf(r).map_err(Stop::Mac)?;
        sys::sys_init(r);
        trx::trx_init(r).map_err(Stop::Mac)?;
        pcie::post_init(r).map_err(Stop::Pcie)?;
        init_cmds::ofld_cfg(r, &mut rings.ch12, fw).map_err(Stop::Send)
    };
    let out = run(rings, fw);
    if out.is_err() {
        let _ = pwr::switch(r, false, &board);
    }
    out
}
