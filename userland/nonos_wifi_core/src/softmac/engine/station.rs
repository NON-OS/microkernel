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

use crate::address::Addresses;
use crate::pmksa::PmksaCache;
use crate::regdom::Domain;
use crate::softmac::radio::{Env, Radio};
use crate::softmac::roam::Look;
use crate::softmac::scanner::Scanner;
use crate::softmac::session::Session;
use crate::softmac::stats::LinkStats;
use alloc::vec::Vec;

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
    /// PMKSAs SAE joins made, for this boot only.
    pub(super) pmksa: PmksaCache,
    /// A look for a better AP of the session's network, while one runs.
    pub(super) look: Option<Look>,
    /// APs a roam to failed, never tried again this boot.
    pub(super) barred: Vec<[u8; 6]>,
    /// The code of the last roam that failed, 0 after one that succeeded.
    pub(super) last_roam: i32,
    /// The secret each network's address is made from; `None` without
    /// randomness, and then no join is made.
    pub(super) addresses: Option<Addresses>,
    /// Set while the radio is turned off: no transmit path sends anything
    /// (E-72). A held session is frozen, not torn down, and resumes on.
    pub(super) off: bool,
}
