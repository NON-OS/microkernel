// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_mac_hfc_init` (Linux v6.12 rtw89 `mac.c`) with the `_ax` helpers.
//! For the download (`en` false, `h2c_en` true) only the command channel's
//! precommitted pages are set; for service every channel's budget, the
//! public pages and the mix, then the public page accounting read back.

use super::hfc_cfg::*;
use super::HfcFail;
use crate::regs::{field, get, Mmio};

fn func_en(r: &impl Mmio, en: bool, h2c_en: bool) {
    let mut v = r.read32(R_AX_HCI_FC_CTRL) & !(HCI_FC_EN | HCI_FC_CH12_EN);
    if en {
        v |= HCI_FC_EN;
    }
    if h2c_en {
        v |= HCI_FC_CH12_EN;
    }
    r.write32(R_AX_HCI_FC_CTRL, v);
}

pub fn hfc_init(r: &impl Mmio, en: bool, h2c_en: bool) -> Result<(), HfcFail> {
    let (ch011, h2c, wp07, wp811, ch011_full, h2c_full, wp07_full, wp811_full) = PREC;
    func_en(r, false, false);
    if !en && h2c_en {
        r.write32(R_AX_CH_PAGE_CTRL, field(PREC_CH12, h2c as u32));
        let v = r.read32(R_AX_HCI_FC_CTRL) & !CH12_FULL_COND;
        r.write32(R_AX_HCI_FC_CTRL, v | field(CH12_FULL_COND, h2c_full as u32));
        func_en(r, false, true);
        return Ok(());
    }
    let (g0, g1, pub_max, wp_thrd) = PUB;
    for (i, c) in CH_CFG.iter().enumerate().filter(|(i, _)| DMA_CH_MASK & (1 << i) == 0) {
        if (c.min != 0 && c.min < ch011) || c.max > pub_max {
            return Err(HfcFail::ChCfg(i as u8));
        }
        let grp = if c.grp1 { PG_GRP } else { 0 };
        let v = field(PG_MIN, c.min as u32) | field(PG_MAX, c.max as u32) | grp;
        r.write32(R_AX_ACH0_PAGE_CTRL + i as u32 * 4, v);
    }
    if g0 + g1 != pub_max {
        return Err(HfcFail::PubCfg);
    }
    r.write32(R_AX_PUB_PAGE_CTRL1, field(G0, g0 as u32) | field(G1, g1 as u32));
    r.write32(R_AX_WP_PAGE_CTRL2, field(PG_MIN, wp_thrd as u32));
    r.write32(R_AX_CH_PAGE_CTRL, field(PREC_CH011, ch011 as u32) | field(PREC_CH12, h2c as u32));
    r.write32(R_AX_PUB_PAGE_CTRL2, field(G0, pub_max as u32));
    r.write32(R_AX_WP_PAGE_CTRL1, field(PREC_CH011, wp07 as u32) | field(PREC_CH12, wp811 as u32));
    let keep =
        !(HCI_FC_MODE | WD_FULL_COND | CH12_FULL_COND | WP_CH07_FULL_COND | WP_CH811_FULL_COND);
    let v = (r.read32(R_AX_HCI_FC_CTRL) & keep)
        | field(HCI_FC_MODE, MODE_POH)
        | field(WD_FULL_COND, ch011_full as u32)
        | field(CH12_FULL_COND, h2c_full as u32)
        | field(WP_CH07_FULL_COND, wp07_full as u32)
        | field(WP_CH811_FULL_COND, wp811_full as u32);
    r.write32(R_AX_HCI_FC_CTRL, v);
    func_en(r, en, h2c_en);
    crate::wait::delay_ms(1);
    let info1 = r.read32(R_AX_PUB_PAGE_INFO1);
    let aval = get(r.read32(R_AX_PUB_PAGE_INFO2), PUB_AVAL);
    let max = get(r.read32(R_AX_PUB_PAGE_CTRL2), G0);
    if en && get(info1, G0) + get(info1, G1) + aval != max {
        return Err(HfcFail::PubInfo);
    }
    Ok(())
}
