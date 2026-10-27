// NONOS Operating System (AGPL-3.0-or-later)

//! `Radio` for the RTL8852BE. Each method is the rtw89 path mac80211 takes
//! for the same step: `tune` is `rtw89_set_channel`, `set_bss(None)` a
//! software scan's start, `set_bss(Some)` the BSSID
//! change and the AP added as a station, `associated` the association or
//! the disconnect; `send` and `recv` the rings. No crypto runs on the chip.

use super::counts::bump;
use super::Station;
use crate::chan::Chan;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::{Association, Radio, RxInfo};

impl<R: Mmio, M: DmaMem> Radio for Station<R, M> {
    fn mac(&self) -> [u8; 6] {
        self.chip.vif.sma
    }

    fn set_mac(&mut self, mac: [u8; 6]) -> bool {
        self.chip.vif.sma = mac;
        let ok = self.chip.cam_update(mac).is_ok();
        if !ok {
            bump(&mut self.counts.cmd_fail);
        }
        ok
    }

    fn bands(&self) -> (bool, bool) {
        (true, true)
    }

    fn tune(&mut self, band: Band, channel: u8) -> bool {
        let five = match band {
            Band::Ghz2 => false,
            Band::Ghz5 => true,
            Band::Ghz6 => return false,
        };
        let Some(c) = Chan::new(five, channel) else { return false };
        let ok = self.chip.tune_to(c).is_ok();
        if !ok {
            bump(&mut self.counts.tune_fail);
        }
        ok
    }

    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.push(mpdu)
    }

    fn recv(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        self.pull(out)
    }

    fn set_bss(&mut self, bssid: Option<[u8; 6]>) {
        self.point(bssid);
    }

    fn associated(&mut self, assoc: Option<&Association>) {
        self.settle(assoc);
    }

    fn diag(&self) -> [u32; 5] {
        self.words()
    }
}
