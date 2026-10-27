/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Tuning: back on the associated BSS's channel its context holds it and
//! any request is given back; elsewhere a request is made, a `JOIN` while
//! the station points at an access point it has not associated with yet,
//! and renewed before it lapses while the radio stays.

use nonos_wifi_core::regdom::Band;

use super::roc::{JOIN_MS, LISTEN_MS};
use super::Mt7921;

impl Mt7921 {
    pub(super) fn tune_to(&mut self, band: Band, ch: u8) -> bool {
        let has = match band {
            Band::Ghz2 => self.caps.ghz2,
            Band::Ghz5 => self.caps.ghz5,
            Band::Ghz6 => false,
        };
        if self.dead || !has {
            return false;
        }
        if self.home == Some((band, ch)) {
            self.release();
            self.at = self.home;
            return true;
        }
        let join = self.peer.is_some() && self.joined.is_none();
        let ok = self.acquire(band, ch, join);
        self.at = ok.then_some((band, ch));
        ok
    }

    /// Renew a request that is near its end while the radio stays there.
    pub(super) fn renew(&mut self, now: u64) {
        let (Some(h), Some((band, ch))) = (self.roc, self.at) else { return };
        let life = u64::from(if h.join { JOIN_MS } else { LISTEN_MS });
        if now.saturating_sub(h.since) >= life * 7 / 10 && !self.acquire(band, ch, h.join) {
            self.at = None;
        }
    }
}
