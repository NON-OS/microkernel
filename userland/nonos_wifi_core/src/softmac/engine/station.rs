// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The shared soft-MAC station: one per chip driver, holding the radio the
//! driver built, the clock and randomness, the regulatory domain, the
//! background scanner and the session. The driver's serve loop hands it each
//! request it receives and calls `tick` between them; nothing else is the
//! driver's to do above its registers.

use crate::regdom::{Domain, WORLD};
use crate::softmac::radio::{Env, Radio};
use crate::softmac::scanner::Scanner;
use crate::softmac::session::Session;
use crate::softmac::stats::LinkStats;

pub struct SoftStation<R: Radio, E: Env> {
    /// `None` when bring-up stopped short of a working radio: the station
    /// still serves, answering every link request with "down" and the status
    /// op with how far bring-up got.
    pub(super) radio: Option<R>,
    pub(super) env: E,
    pub(super) stage: u8,
    pub(super) domain: Domain,
    pub(super) scanner: Scanner,
    pub(super) session: Option<Session>,
    pub(super) stats: LinkStats,
}

impl<R: Radio, E: Env> SoftStation<R, E> {
    /// `stage` is the driver's bring-up stage byte, 0 for ready.
    pub fn new(radio: Option<R>, env: E, stage: u8) -> Self {
        let (scanner, stats) = (Scanner::new(), LinkStats::default());
        Self { radio, env, stage, domain: WORLD, scanner, session: None, stats }
    }

    /// Run under `domain` instead of the world domain.
    pub fn set_domain(&mut self, domain: Domain) {
        self.domain = domain;
    }

    // A session over, by the AP's word, silence or the person's: forget it,
    // open the receive filter and let the sweep start again.
    pub(super) fn end_session(&mut self) {
        self.session = None;
        if let Some(r) = self.radio.as_mut() {
            r.set_bss(None);
        }
        self.scanner.resume();
    }
}
