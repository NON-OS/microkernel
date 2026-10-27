// NONOS Operating System (AGPL-3.0-or-later)

//! The PHY half of `rtw89_core_start` before coexistence (Linux v6.12
//! rtw89 `core.c`): `rtw89_chip_reset_bb_rf` (BB and RF disabled, enabled
//! again), `rtw89_phy_init_bb_reg`, and `rtw89_phy_init_rf_reg`, whose
//! direct RF registers then go to the firmware page by page
//! (`rtw89_phy_config_rf_reg_fw`). rtw89 warns and goes on when a page
//! cannot be sent or a path has too many words; this reports both.

use super::probe::Probe;
use super::Stop;
use crate::dma::DmaMem;
use crate::h2c::{init_cmds, Fw};
use crate::mac::sys;
use crate::phy::init_bb::init_bb;
use crate::phy::init_rf::{init_rf, ForFw};
use crate::phy::state::{EfuseGain, Phy};
use crate::regs::Mmio;
use crate::ring::tx::TxRing;

/// The PHY state from what the probe read.
pub fn phy_of(p: &Probe) -> Phy {
    let (e, c) = (&p.efuse.efuse, &p.efuse.phycap);
    let efuse_gain = EfuseGain {
        offset: e.gain,
        offset_valid: e.gain_valid,
        comp: c.gain_comp,
        comp_valid: c.gain_comp_valid,
        ..EfuseGain::default()
    };
    Phy { rfe: e.rfe_type, cv: p.cv, efuse_gain, ..Phy::default() }
}

/// Whether every path's RF words reached the firmware.
pub fn phy_init<M: DmaMem>(
    r: &impl Mmio,
    h2c: &mut TxRing<M>,
    fw: &mut Fw,
    phy: &mut Phy,
) -> Result<bool, Stop> {
    sys::disable_bb_rf(r).map_err(Stop::Mac)?;
    sys::enable_bb_rf(r).map_err(Stop::Mac)?;
    init_bb(r, phy.rfe, phy.cv, &mut phy.gain).map_err(Stop::Table)?;
    let mut all_sent = true;
    for path in 0..2u8 {
        let words = match init_rf(r, path, phy.rfe, phy.cv).map_err(Stop::Table)? {
            ForFw::Words(w) => w,
            ForFw::TooMany => {
                all_sent = false;
                continue;
            }
        };
        // A page that fails ends the path's pages, as in rtw89.
        for (page, chunk) in words.chunks(init_cmds::RF_PAGE_WORDS).enumerate() {
            if init_cmds::rf_reg_page(r, h2c, fw, path, page as u8, chunk).is_err() {
                all_sent = false;
                break;
            }
        }
    }
    Ok(all_sent)
}
