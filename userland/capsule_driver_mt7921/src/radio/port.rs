/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The shared station's view of the card. CCMP-128 alone is offered (the
//! trait's default): the station seals and opens every frame in software.

use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::{Association, Radio, RxInfo};

use super::Mt7921;

impl Radio for Mt7921 {
    fn mac(&self) -> [u8; 6] {
        self.mac
    }

    fn set_mac(&mut self, mac: [u8; 6]) -> bool {
        self.change_mac(mac)
    }

    fn bands(&self) -> (bool, bool) {
        (self.caps.ghz2, self.caps.ghz5)
    }

    fn tune(&mut self, band: Band, channel: u8) -> bool {
        self.tune_to(band, channel)
    }

    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.send_frame(mpdu)
    }

    fn recv(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        self.recv_frame(out)
    }

    fn set_bss(&mut self, bssid: Option<[u8; 6]>) {
        self.bss(bssid)
    }

    fn service(&mut self, now_ms: u64) {
        self.upkeep(now_ms)
    }

    fn associated(&mut self, assoc: Option<&Association>) {
        self.assoc(assoc)
    }

    fn diag(&self) -> [u32; 5] {
        self.diag_words()
    }
}
