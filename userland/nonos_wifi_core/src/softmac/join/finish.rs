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

//! A join that reached Connected becomes the keys a session starts from.

use super::report::Outcome;
use crate::mlme::Mlme;
use crate::rsn::TemporalKey;

/// The keys of a connected join, or `Refused` if the machine says connected
/// but holds no key of its pairwise cipher's length (which a correct
/// machine never does).
pub(super) fn finish(m: &Mlme) -> Outcome {
    let (Some(tk), Some(sup), Some(akm)) = (m.tk(), m.supplicant(), m.akm()) else {
        return Outcome::Refused;
    };
    let Some(tk) = TemporalKey::new(sup.pairwise_cipher(), tk) else {
        return Outcome::Refused;
    };
    Outcome::Joined { bssid: m.bssid(), channel: m.channel(), tk, akm, supplicant: sup }
}
