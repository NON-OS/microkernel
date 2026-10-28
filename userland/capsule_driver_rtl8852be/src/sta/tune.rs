// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_set_channel` (Linux v6.12 rtw89 `core.c`) for a 20 MHz channel,
//! wrapped in `rtw8852b_set_channel_help` (`rtw8852b.c`): TX scheduler
//! paused, PPDU status off, TSSI tracking held, the ADC FIFO held and the
//! BB held in reset across the MAC, BB and RF channel set and the TX power
//! for it; then all of it let go in reverse. rtw89 then runs the TSSI set-up
//! for a new band (`rtw8852b_tssi_scan`); TSSI is not ported, so nothing
//! follows.

use super::chip::Chip;
use crate::chan::Chan;
use crate::dma::DmaMem;
use crate::mac::chan::{ppdu_status, set_channel_mac};
use crate::mac::sch::{self, SchFail};
use crate::phy::bb::Bb;
use crate::phy::{ch_bb, ch_rf, txpwr};
use crate::regs::Mmio;
use crate::wait::delay_ms;

/// `rtw8852b_tssi_cont_en` on both paths: (R_P0/P1_TXPW_RSTB MANON,
/// R_P0/P1_TSSI_TRK EN), set to hold, cleared to run.
const TSSI_HOLD: [(u32, u32); 4] =
    [(0x58dc, 1 << 30), (0x5818, 1 << 30), (0x78dc, 1 << 30), (0x7818, 1 << 30)];

fn help(r: &impl Mmio, five: bool, enter: bool) {
    let bb = Bb(r);
    if enter {
        let _ = ppdu_status(r, false);
        for (a, m) in TSSI_HOLD {
            bb.mask(a, m, 1);
        }
        bb.mask(0x20fc, 0xff00_0000, 0xf); // R_ADC_FIFO reset
        delay_ms(1); // fsleep(40)
        bb.mask(0x2344, 1 << 31, 1); // R_RXCCA DIS
        bb.mask(0xc3c, 0x200, 1); // R_PD_CTRL HIT_DIS
        bb.mask(0x1200, 0x7000_0000, 0x7); // S0 HW SI off
        bb.mask(0x3200, 0x7000_0000, 0x7);
        delay_ms(1);
        bb.mask(0x704, 0x2, 0); // R_RSTB_ASYNC
    } else {
        let _ = ppdu_status(r, true);
        bb.mask(0x20fc, 0xff00_0000, 0);
        for (a, m) in TSSI_HOLD {
            bb.mask(a, m, 0);
        }
        bb.mask(0x1200, 0x7000_0000, 0);
        bb.mask(0x3200, 0x7000_0000, 0);
        bb.mask(0x704, 0x2, 1);
        if !five {
            bb.mask(0x2344, 1 << 31, 0);
        }
        bb.mask(0xc3c, 0x200, 0);
    }
}

impl<R: Mmio, M: DmaMem> Chip<R, M> {
    pub fn tune_to(&mut self, c: Chan) -> Result<(), SchFail> {
        let was = sch::stop(&self.r, &mut self.fw)?;
        help(&self.r, c.five, true);
        set_channel_mac(&self.r, c.five);
        ch_bb::set_channel_bb(&self.r, &self.phy, c);
        ch_rf::set_channel_rf(&self.r, c.ch);
        txpwr::set_txpwr(&self.r, c);
        help(&self.r, c.five, false);
        self.chan = Some(c);
        sch::resume(&self.r, &mut self.fw, was)
    }
}
