/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Tuning, by asking the firmware's channel manager for the channel and
//! waiting up to a second for its grant (Linux v6.12 `mt7921_set_roc`,
//! which waits `HZ`), reading the data ring meanwhile too. A grant is this
//! request's when it names its token or its channel: Linux takes any grant
//! (`mt7921_mcu_uni_roc_event` reads neither), and a firmware that does not
//! echo the token must not leave every channel ungranted. A request for a join is a `JOIN` held a second, as
//! `mt7921_mgd_prepare_tx` asks; one to listen is a `ROC` held half a
//! second. A request is renewed before it lapses for as long as the
//! station stays on its channel; once associated, the BSS's own context
//! holds its channel and no request is needed there.

use nonos_wifi_core::regdom::Band;

use super::Mt7921;
use crate::clock::{now_ms, until_ms};
use crate::mcu::cmd::ROC;
use crate::vif::roc::{abort, acquire, Req};

/// A channel request held: its token, whether it is a join's, and since.
#[derive(Clone, Copy)]
pub struct Held {
    pub token: u8,
    pub join: bool,
    pub since: u64,
}

const GRANT_MS: u64 = 1000;
pub const JOIN_MS: u32 = 1000;
pub const LISTEN_MS: u32 = 500;

impl Mt7921 {
    /// Give a held request back (`mt7921_mcu_abort_roc`).
    pub(super) fn release(&mut self) {
        if let Some(h) = self.roc.take() {
            self.card.cmd(ROC, &abort(h.token), false);
        }
    }

    /// Ask for `ch` and wait for its grant; whether it came.
    pub(super) fn acquire(&mut self, band: Band, ch: u8, join: bool) -> bool {
        self.release();
        self.token = self.token.wrapping_add(1);
        let token = self.token;
        let (req, ms) = if join { (Req::Join, JOIN_MS) } else { (Req::Listen, LISTEN_MS) };
        self.card.mcu.grant = None;
        if !self.card.cmd(ROC, &acquire(token, ch, band == Band::Ghz5, req, ms), false) {
            return false;
        }
        let granted = until_ms(GRANT_MS, || {
            self.card.pump();
            self.reap();
            self.card.mcu.grant.is_some_and(|g| g.token == token || g.channel == ch)
        });
        if !granted {
            self.count.missed = self.count.missed.wrapping_add(1);
            self.card.cmd(ROC, &abort(token), false);
            return false;
        }
        self.count.grants = self.count.grants.wrapping_add(1);
        self.roc = Some(Held { token, join, since: now_ms() });
        true
    }
}
