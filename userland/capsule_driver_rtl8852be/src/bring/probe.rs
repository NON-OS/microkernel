// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_chip_info_setup` (Linux v6.12 rtw89 `core.c`) for the 8852B:
//! the cut, the firmware suit for it, then `rtw89_chip_efuse_info_setup`:
//! a partial init (power, pre-inits, download), the efuse and phycap maps,
//! the firmware's PHY capabilities (`rtw89_mac_setup_phycap`, over the
//! register mailbox), and power off again.

use super::{partial_init, Stop};
use crate::dma::DmaMem;
use crate::efuse;
use crate::fw;
use crate::h2c::reg::{msg, C2H_PHY_CAP, FUNC_GET_FEATURE};
use crate::h2c::Fw;
use crate::pwr::{self, Board};
use crate::regs::{get, Mmio};
use crate::ring::Rings;

/// What the firmware says about the radio: streams and antennas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Caps {
    pub tx_nss: u8,
    pub rx_nss: u8,
    /// `hal.antenna_rx` is RF_B: the board has one RX antenna, on path B.
    pub rx_b_only: bool,
}

/// The 8852B's `tx_nss` and `rx_nss`.
pub const CHIP_NSS: u8 = 2;

/// `rtw89_mac_setup_phycap`'s reading of the PHY_CAP answer.
pub fn caps_of(w: [u32; 4]) -> Caps {
    let (tx, rx) = (get(w[1], 0xff) as u8, get(w[0], 0xff_0000) as u8);
    let pick = |n: u8| if n == 0 { CHIP_NSS } else { n.min(CHIP_NSS) };
    Caps { tx_nss: pick(tx), rx_nss: pick(rx), rx_b_only: get(w[3], 0xff_0000) == 1 }
}

pub struct Probe {
    pub cv: u8,
    pub efuse: efuse::Board,
    pub caps: Caps,
}

pub fn probe<M: DmaMem>(r: &impl Mmio, rings: &mut Rings<M>, fw: &mut Fw) -> Result<Probe, Stop> {
    let (cv, _acv) = pwr::chip_ver(r);
    let (suit, hdr) = fw::prepare(fw::IMAGE, cv).map_err(Stop::Firmware)?;
    let first = Board { cv, ..Board::default() };
    partial_init(r, rings, fw, &first, suit, &hdr)?;
    let efuse = efuse::read(r, cv).map_err(Stop::Efuse)?;
    let w =
        msg(r, &mut fw.mailbox, FUNC_GET_FEATURE, [0; 4], 0, C2H_PHY_CAP).map_err(Stop::Mailbox)?;
    let caps = caps_of(w);
    pwr::switch(r, false, &board_of(cv, &efuse)).map_err(Stop::Power)?;
    Ok(Probe { cv, efuse, caps })
}

/// The power sequence's view of the board once the efuse is read.
pub fn board_of(cv: u8, e: &efuse::Board) -> Board {
    Board {
        rfe_type: Some(e.efuse.rfe_type),
        efuse_valid: e.valid,
        power_k_valid: e.phycap.power_k_valid,
        cv,
    }
}
