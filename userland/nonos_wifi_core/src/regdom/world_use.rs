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

//! What the world domain allows on one channel number, for a driver that
//! takes the world-safe default without a scan plan: the station may transmit
//! first only where every country allows it, listens elsewhere, and never
//! uses a channel the domain closes. Read from `WORLD`, the one table.

use super::band::{Band, CHANNELS_2GHZ, CHANNELS_5GHZ};
use super::permit::Permit;
use super::world::WORLD;

/// How a channel may be used.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Use {
    /// The station may transmit first: probe, then join.
    Active,
    /// Listen only; join an access point only once its beacon was heard.
    Passive,
}

/// What the world domain allows on `channel`, a 2.4 or 5 GHz channel
/// number; `None` where it allows nothing or names no such channel.
pub fn world(channel: u8) -> Option<Use> {
    if !CHANNELS_2GHZ.contains(&channel) && !CHANNELS_5GHZ.contains(&channel) {
        return None;
    }
    match WORLD.permit(Band::of_legacy(channel), channel) {
        Permit::Active => Some(Use::Active),
        Permit::Passive { .. } => Some(Use::Passive),
        Permit::Disabled => None,
    }
}

/// The highest EIRP the world domain allows on `channel`, in dBm.
pub fn max_eirp_dbm(channel: u8) -> Option<u8> {
    world(channel)?;
    let dbm = WORLD.max_eirp_dbm(Band::of_legacy(channel), channel)?;
    u8::try_from(dbm).ok()
}
