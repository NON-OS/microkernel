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

mod answer;
mod bank;
mod battery;
mod cell;
mod charge;
mod charged;
mod chars;
mod descs;
mod dispatch;
mod enable;
mod hid;
mod leds;
mod mtu;
mod notify;
mod phase;
mod ready;
mod refs;
mod refs_found;
mod refused;
mod reports;
mod services;
mod sources;
mod start;
mod state;
mod step;
mod timer;
mod types;
mod upgrade;
mod upgrade_answer;
mod upgrade_leds;
mod upgrade_take;

pub use hid::REPORTS_MAX;
pub use mtu::MTU_DEFAULT;
pub use phase::Phase;
pub use state::Host;
pub use step::{Outcome, Step};
pub use types::{Battery, Kind, Source, MAX_DECLS, MAX_REQUESTS, MAX_SERVICES, TIMEOUT_MS};
