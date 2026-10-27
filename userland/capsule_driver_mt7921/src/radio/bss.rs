/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The station pointing the radio at an access point, or away, and
//! changing its address. Pointing it at one makes the AP's station record
//! (`mt7921_mac_sta_add`, before authentication) and holds the channel by
//! a `JOIN` request (`mt7921_mgd_prepare_tx`). While associated the record
//! stays: the station opens its filter for a look at other channels and
//! points it back, and only the end of the association removes it. An
//! address changes as an interface does under mac80211, down and up again
//! under the new one (`mt76_connac_mcu_uni_add_dev`), never while
//! associated.

use super::Mt7921;
use crate::vif::own::record;
use crate::vif::sta::Peer;
use crate::vif::{self, AP_WCID};

impl Mt7921 {
    pub(super) fn bss(&mut self, bssid: Option<[u8; 6]>) {
        if self.joined.is_some() {
            return;
        }
        let Some(b) = bssid else {
            self.drop_peer();
            self.release();
            return;
        };
        if self.peer.is_none_or(|p| p.addr != b) {
            self.drop_peer();
            let ghz5 = self.ghz5();
            // Before association the band's whole legacy set, its
            // mandatory rates basic: 1 to 11 Mb/s, or 6, 12 and 24.
            let rates = if ghz5 { (0xff, 0x15) } else { (0xfff, 0xf) };
            let peer = Peer { addr: b, wcid: AP_WCID, aid: 0, ghz5, rates };
            let made = record(&mut self.card, &peer, true);
            self.note(made);
            self.peer = Some(peer);
        }
        if let Some((band, ch)) = self.at {
            self.acquire(band, ch, true);
        }
    }

    fn drop_peer(&mut self) {
        if let Some(p) = self.peer.take() {
            let ok = record(&mut self.card, &p, false);
            self.note(ok);
        }
    }

    pub(super) fn change_mac(&mut self, mac: [u8; 6]) -> bool {
        if mac == self.mac || self.dead {
            return mac == self.mac && !self.dead;
        }
        if self.joined.is_some() || !vif::remove(&mut self.card, self.mac) {
            return false;
        }
        let up = vif::add(&mut self.card, mac);
        self.mac = mac;
        let heard = vif::listen(&mut self.card);
        let rec = self.peer.is_none_or(|p| record(&mut self.card, &p, true));
        if let (true, Some((band, ch))) = (up, self.at) {
            self.acquire(band, ch, self.peer.is_some());
        }
        self.note(up && heard && rec);
        up
    }
}
