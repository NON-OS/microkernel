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

//! The regulatory domain a station runs under, the world domain by default.
//! With no country the person confirmed, a station may transmit first only
//! where every country allows it: 2.4 GHz channels 1 to 11. Channels 12 and
//! 13, and the 5 GHz channels of the world list, are passive: listened on, and
//! joined once an access point's beacon was heard there, but never probed.
//! Channel 14, the 5 GHz channels above 165 and anything else are never used.
//! A beacon's channel is taken for a join only where it could be true
//! (`join_channel`).

mod band;
mod channel;
mod channel_set;
mod country;
mod domain;
mod permit;
mod rule;
mod scan_plan;
mod scan_step;
mod world;
mod world_use;
pub use band::{channel_mhz, Band, CHANNELS_2GHZ, CHANNELS_5GHZ};
pub use channel::join_channel;
pub use channel_set::ChannelSet;
pub use country::{parse_country, Country, EID_COUNTRY};
pub use domain::Domain;
pub use permit::Permit;
pub use rule::{Rule, RuleFlags};
pub use scan_plan::ScanPlan;
pub use scan_step::{ScanMode, ScanStep, ACTIVE_DWELL_MS, PASSIVE_DWELL_MS};
pub use world::WORLD;
pub use world_use::{max_eirp_dbm, world, Use};
