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

//! A station starting, its domain set, and a session ending.

use super::station::SoftStation;
use crate::pmksa::PmksaCache;
use crate::regdom::{Domain, WORLD};
use crate::softmac::radio::{Env, Radio};
use crate::softmac::scanner::Scanner;
use crate::softmac::stats::LinkStats;
use crate::vault::Vault;
use alloc::vec::Vec;

/// Barred candidates kept; past this the oldest goes.
pub const BARRED_MAX: usize = 16;

impl<R: Radio, E: Env> SoftStation<R, E> {
    /// `stage` is the driver's bring-up stage byte, 0 for ready.
    pub fn new(radio: Option<R>, mut env: E, stage: u8) -> Self {
        let (scanner, stats) = (Scanner::new(), LinkStats::default());
        // The PMKSA tag key, fresh at each start; with no randomness, no
        // PMKSA is kept this boot.
        let pmksa = PmksaCache::new(Vault::fill(|k| env.random(k)));
        let (session, look, barred, last_roam) = (None, None, Vec::new(), 0);
        let domain = WORLD;
        Self { radio, env, stage, domain, scanner, session, stats, pmksa, look, barred, last_roam }
    }

    /// Run under `domain` instead of the world domain.
    pub fn set_domain(&mut self, domain: Domain) {
        self.domain = domain;
    }

    // A session over, by the AP's word, silence or the person's: forget it,
    // open the receive filter and let the sweep start again.
    pub(super) fn end_session(&mut self) {
        let had = self.session.take().is_some();
        self.look = None;
        if let Some(r) = self.radio.as_mut() {
            if had {
                r.associated(None);
            }
            r.set_bss(None);
        }
        self.scanner.resume();
    }
}
