// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_phy_dm_init` (Linux v6.12 rtw89 `phy.c`) as far as it programs
//! the 8852B: BB set-up and RPL bases, the DIG reset, the crystal cap and
//! CFO compensation, EDCCA, the NCTL block and table, the RF calibrations
//! of `rtw8852b_rfk_init`, the TX power reference, the power trims, and
//! both RX paths on rtw89's default channel (2 GHz channel 1). Then the
//! rest of `rtw89_core_start` that touches the chip: PPDU status reports
//! on, the RTS thresholds, and the firmware log left off.

use super::probe::Probe;
use super::Stop;
use crate::chan::Chan;
use crate::dma::DmaMem;
use crate::h2c::{init_cmds, Fw};
use crate::mac::chan::ppdu_status;
use crate::phy::state::Phy;
use crate::phy::{dm_hw, nctl, rx_path, trim, txpwr_ref};
use crate::regs::{Mmio, Rw};
use crate::rfk::{self, Rfk};
use crate::ring::tx::TxRing;

/// rtw89's channel before any is set: `rtw89_config_default_chandef`.
pub const DEFAULT_CHAN: Chan = Chan { five: false, ch: 1 };
const R_AX_AGG_LEN_HT_0: u32 = 0xc614;

pub fn dm_init<M: DmaMem>(
    r: &impl Mmio,
    h2c: &mut TxRing<M>,
    fw: &mut Fw,
    phy: &mut Phy,
    rfk: &mut Rfk,
    p: &Probe,
) -> Result<(), Stop> {
    dm_hw::sethw(r, phy);
    dm_hw::dig_reset(r, p.cv);
    dm_hw::cfo_init(r, p.efuse.efuse.xtal_k).map_err(|_| Stop::Xtal)?;
    dm_hw::edcca_init(r);
    nctl::init_rf_nctl(r, phy.rfe, phy.cv).map_err(Stop::Nctl)?;
    rfk::init(r, fw, rfk).map_err(Stop::Sch)?;
    txpwr_ref::set_txpwr_ref(r);
    trim::power_trim(r, p.efuse.phycap.thermal_trim, p.efuse.phycap.pa_bias_trim);
    rx_path::cfg_txrx_path(r, phy, DEFAULT_CHAN, p.caps.rx_nss);
    ppdu_status(r, true).map_err(Stop::Mac)?;
    // rtw89_mac_update_rts_threshold with no threshold set: 88 us >> 5 and
    // 4080 bytes >> 4, each capped at 255.
    r.mask16(R_AX_AGG_LEN_HT_0, 0xff00, 88 >> 5);
    r.mask16(R_AX_AGG_LEN_HT_0, 0xff, 255);
    init_cmds::fw_log_off(r, h2c, fw).map_err(Stop::Send)
}
