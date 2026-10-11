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
mod band;
mod channel_set;
mod country;
mod domain;
mod permit;
mod rule;
mod scan_plan;
mod scan_step;
mod world;
pub use band::{channel_mhz, Band, CHANNELS_2GHZ, CHANNELS_5GHZ};
pub use channel_set::ChannelSet;
pub use country::{parse_country, Country, EID_COUNTRY};
pub use domain::Domain;
pub use permit::Permit;
pub use rule::{Rule, RuleFlags};
pub use scan_plan::ScanPlan;
pub use scan_step::{ScanMode, ScanStep, ACTIVE_DWELL_MS, PASSIVE_DWELL_MS};
pub use world::WORLD;
