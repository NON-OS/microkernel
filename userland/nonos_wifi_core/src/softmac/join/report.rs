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

//! How a join ended and how far it got. The counters turn a bare timeout
//! into a diagnosis the panel can show: one frame sent and none received
//! points at transmit, data frames with no EAPOL among them at framing, a
//! deauthentication after the first EAPOL at a handshake frame lost here.

use super::counters::Counters;
use crate::mlme::{Mlme, MlmeFailure, MlmeState};
use crate::wpa::akm::Akm;
use crate::wpa::supplicant::Supplicant;

// Built once per join and moved straight into the session, so the size gap
// between Joined and the bare endings costs nothing worth a box.
#[allow(clippy::large_enum_variant)]
pub enum Outcome {
    /// Associated: the AP, its channel, the pairwise key and the supplicant
    /// that keeps answering the AP's group key handshakes.
    Joined { bssid: [u8; 6], channel: u8, tk: [u8; 16], akm: Akm, supplicant: Supplicant },
    /// Refused, or the handshake broke; `Report::failure` says where.
    Refused,
    /// The AP stopped answering before the join finished.
    TimedOut,
}

pub struct Report {
    pub outcome: Outcome,
    pub counters: Counters,
    /// The state reached: 0 scanning to 4 connected, 5 failed.
    pub state: u8,
    pub failure: Option<MlmeFailure>,
    pub akm: Option<Akm>,
}

pub(super) fn state_code(s: MlmeState) -> u8 {
    match s {
        MlmeState::Scanning => 0,
        MlmeState::Authenticating => 1,
        MlmeState::Associating => 2,
        MlmeState::FourWay => 3,
        MlmeState::Connected => 4,
        MlmeState::Failed => 5,
    }
}

pub(super) fn report(m: &Mlme, outcome: Outcome, counters: Counters) -> Report {
    let state = state_code(m.state());
    Report { outcome, counters, state, failure: m.failure(), akm: m.akm() }
}

/// The keys of a connected join, or `Refused` if the machine says connected
/// but holds no 16-octet key (which a correct machine never does).
pub(super) fn finish(m: &Mlme) -> Outcome {
    let (Some(tk), Some(sup), Some(akm)) = (m.tk(), m.supplicant(), m.akm()) else {
        return Outcome::Refused;
    };
    let Ok(tk) = <[u8; 16]>::try_from(tk) else {
        return Outcome::Refused;
    };
    Outcome::Joined { bssid: m.bssid(), channel: m.channel(), tk, akm, supplicant: sup }
}
