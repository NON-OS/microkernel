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

//! The regulatory domain a join runs under. The station answers only an
//! access point it heard, so a channel the domain leaves passive is joinable;
//! one it closes is not, and an AP heard there is treated as not heard. The
//! default is the world domain until the driver is given another.

use super::state::Mlme;
use crate::regdom::{Band, Domain, Permit};

impl Mlme {
    /// Join under `domain` instead of the world domain.
    pub fn set_domain(&mut self, domain: Domain) {
        self.domain = domain;
    }

    /// The domain the join runs under.
    pub fn domain(&self) -> Domain {
        self.domain
    }

    // A beacon without a DS Parameter Set names no channel (5 GHz APs may omit
    // it); it was heard on the channel the driver tuned, which came from the
    // domain's scan plan, so it stands.
    pub(super) fn channel_open(&self, channel: u8) -> bool {
        channel == 0 || self.domain.permit(Band::of_legacy(channel), channel) != Permit::Disabled
    }
}
