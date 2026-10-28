/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The basic rates of the access points heard, read from their beacons and
//! probe responses (Supported and Extended Supported Rates), so the first
//! authentication frame already goes at a rate the chosen one decodes, as
//! mac80211 sets `basic_rates` from the BSS before it authenticates
//! (`ieee80211_prep_connection`). The last sixteen networks are kept, by
//! BSSID; association's own rates take over once it settles.

use nonos_wifi_core::dot11::ies::find_element;

use super::Mt7921;
use crate::mac::rate::basic;

const KEPT: usize = 16;

#[derive(Default)]
pub struct Heard {
    next: usize,
    seen: [([u8; 6], (u8, u8)); KEPT],
}

impl Heard {
    /// The basic rates (lowest, fastest) `bssid` was last heard naming.
    pub fn get(&self, bssid: [u8; 6]) -> Option<(u8, u8)> {
        let known = |s: &&([u8; 6], (u8, u8))| s.0 == bssid && bssid != [0; 6];
        self.seen.iter().find(known).map(|s| s.1)
    }

    fn put(&mut self, bssid: [u8; 6], rates: (u8, u8)) {
        if let Some(s) = self.seen.iter_mut().find(|s| s.0 == bssid) {
            s.1 = rates;
            return;
        }
        self.seen[self.next] = (bssid, rates);
        self.next = (self.next + 1) % KEPT;
    }
}

impl Mt7921 {
    /// Note the basic rates in `f` if it is a beacon or a probe response.
    pub(super) fn learn(&mut self, f: &[u8]) {
        let (Some(&fc), Some(bssid), Some(ies)) = (f.first(), f.get(16..22), f.get(36..)) else {
            return;
        };
        if fc != 0x80 && fc != 0x50 {
            return;
        }
        let each = |id| find_element(ies, id).and_then(|e| e.get(2..)).unwrap_or(&[]);
        let mut rates = [0u8; 16];
        let n = each(1).iter().chain(each(50)).zip(rates.iter_mut()).map(|(r, s)| *s = *r).count();
        if let Some(r) = basic(&rates[..n], self.ghz5()) {
            self.aps.put(bssid.try_into().unwrap_or([0; 6]), r);
        }
    }
}
