/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The station's word that the access point accepted it, or that the
//! association ended. Accepted: the BSS connected on the join's channel,
//! the AP's record in the associated state with the rates the response
//! named, and the interface's own entry; then the join's channel request
//! is given back (`mt7921_mgd_complete_tx`), the BSS's context holding the
//! channel from here. Ended: the record removed, the BSS disconnected.

use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Association;

use super::Mt7921;
use crate::mac::rate;
use crate::vif::bss::Basic;
use crate::vif::own::{associate, depart, Joined};
use crate::vif::sta::Peer;
use crate::vif::AP_WCID;

impl Mt7921 {
    pub(super) fn assoc(&mut self, a: Option<&Association>) {
        let Some(a) = a else { return self.ended() };
        let Some((band, channel)) = self.at else { return };
        let ghz5 = band == Band::Ghz5;
        let rates = &a.rates[..a.n_rates.min(a.rates.len())];
        let peer = Peer {
            addr: a.bssid,
            wcid: AP_WCID,
            aid: a.aid,
            ghz5,
            rates: rate::bitmaps(rates, ghz5),
        };
        // PHY_MODE_A, or PHY_MODE_B | PHY_MODE_G; PHY_TYPE_BIT_OFDM, or
        // HR_DSSS | ERP. No DTIM is known from the response: 1.
        let (phymode, nonht_basic_phy) = if ghz5 { (0x1, 0x8) } else { (0x6, 0x3) };
        let basic = Basic {
            active: true,
            connected: true,
            bssid: a.bssid,
            beacon_int: a.beacon_int,
            dtim: 1,
            phymode,
            nonht_basic_phy,
        };
        let antennas = self.caps.antennas;
        let ok = associate(&mut self.card, &Joined { peer, basic, channel, antennas });
        self.note(ok);
        self.peer = Some(peer);
        let low = rate::lowest(ghz5);
        let set = rate::basic(rates, ghz5).or_else(|| self.aps.get(a.bssid));
        self.joined = Some((basic, set.unwrap_or((low, low))));
        self.home = Some((band, channel));
        self.release();
    }

    fn ended(&mut self) {
        if let (Some((basic, _)), Some(peer)) = (self.joined.take(), self.peer.take()) {
            let ok = depart(&mut self.card, &peer, basic);
            self.note(ok);
        }
        self.home = None;
    }
}
