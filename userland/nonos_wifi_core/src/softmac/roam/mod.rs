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

mod kit;
mod kit_join;
mod listen;
mod look;
mod look_step;
mod null;
mod offer;
mod pick;
mod secret;
mod secret_use;
mod signal;
mod trip;

pub use kit::{Kit, COOLDOWN_MS};
pub use kit_join::roam_policy;
pub use look::{Look, Step, AWAY_CHANNELS, HEARD_MAX, HOME_MS};
pub use null::pm_null;
pub use offer::{group_mgmt_of, offered, Heard};
pub use pick::{pick, Want, HYSTERESIS_DB};
pub use secret::Secret;
pub use signal::{Signal, SAMPLES_MIN, WEAK_DBM};
