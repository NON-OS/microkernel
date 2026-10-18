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
use crate::address::Addresses;
use crate::pmksa::PmksaCache;
use crate::regdom::WORLD;
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
        // The secret each network's address comes from, fresh at each start.
        let addresses = Addresses::for_this_boot(|k| env.random(k));
        let (session, look, barred, last_roam) = (None, None, Vec::new(), 0);
        let domain = WORLD;
        Self {
            radio,
            env,
            stage,
            domain,
            scanner,
            session,
            stats,
            pmksa,
            look,
            barred,
            last_roam,
            addresses,
            off: false,
        }
    }
}
