// NONOS Operating System (AGPL-3.0-or-later)

//! A software scan's start and end, `rtw89_core_scan_start` and
//! `rtw89_core_scan_complete` (Linux v6.12 rtw89 `core.c`), with the
//! receive filter mac80211 asks for while scanning (FIF_BCN_PRBRESP_PROMISC,
//! `rtw89_ops_configure_filter`): the EDCCA levels raised to their maximum
//! and restored after (`rtw89_phy_config_edcca`), the CAM's address set to
//! the scan's. rtw89's TSSI steps around a scan are not ported.

use super::chip::Chip;
use crate::dma::DmaMem;
use crate::h2c::send::SendFail;
use crate::mac::macid::rx_filter;
use crate::phy::bb::Bb;
use crate::regs::Mmio;

/// `R_SEG0R_EDCCA_LVL_V1`: the EDCCA, EDCCA-P and PPDU level bytes.
const EDCCA_LEVEL: u32 = 0x4884;
const MASKS: [u32; 3] = [0xff, 0xff00, 0xff00_0000];
const EDCCA_MAX: u32 = 0xf9;

impl<R: Mmio, M: DmaMem> Chip<R, M> {
    pub fn scan_start(&mut self) -> Result<(), SendFail> {
        if !self.vif.scanning {
            let bb = Bb(&self.r);
            self.vif.edcca_bak = Some(MASKS.map(|m| bb.get(EDCCA_LEVEL, m)));
            for m in MASKS {
                bb.mask(EDCCA_LEVEL, m, EDCCA_MAX);
            }
            self.vif.scanning = true;
        }
        rx_filter(&self.r, true);
        self.cam_update(self.vif.sma)
    }

    pub fn scan_end(&mut self) {
        if !self.vif.scanning {
            return;
        }
        if let Some(bak) = self.vif.edcca_bak.take() {
            let bb = Bb(&self.r);
            for (m, v) in MASKS.iter().zip(bak) {
                bb.mask(EDCCA_LEVEL, *m, v);
            }
        }
        rx_filter(&self.r, false);
        self.vif.scanning = false;
    }
}
