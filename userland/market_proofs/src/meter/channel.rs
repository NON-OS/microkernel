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
//! The margin PAY and FUNDED both keep from a channel's expiry, and the
//! channel's own charge, which both make through MeterState::charged.

#[path = "../../../nonos_nox_meter/src/channel/charge.rs"]
mod charge;
#[path = "../../../nonos_nox_meter/src/channel/margin.rs"]
mod margin;
#[path = "../../../nonos_nox_meter/src/channel/state.rs"]
mod state;

pub use margin::CHARGE_MARGIN;
pub use state::{Channel, MAX_CHANNELS, NO_SIG};
