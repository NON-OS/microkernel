// NONOS Operating System (AGPL-3.0-or-later)

//! `Radio` for the RTL8852BE, each method the rtw89 path mac80211 takes for
//! it: `tune` `rtw89_set_channel`, `set_bss` a scan's start or the AP added,
//! `associated` the association or disconnect, `set_key` `rtw89_ops_set_key`
//! (CCMP-128 in the chip, the station writing the CCMP header).

use super::counts::bump;
use super::Station;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::{Association, ChipKey, Crypto, Radio, RxInfo};

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
        self.retune(band, channel)
    }

    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.push(mpdu, false)
    }

    fn send_keyed(&mut self, mpdu: &[u8]) -> bool {
        self.push(mpdu, true)
    }

    fn crypto(&self) -> Crypto {
        Crypto::HostIv
    }

    fn set_key(&mut self, key: &ChipKey<'_>) -> bool {
        self.key(key)
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
