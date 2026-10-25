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

//! Ports as a hub reports them, and a bring-up looked at every millisecond
//! until it asks for something.

use nonos_usb_hub::Bringup;
use nonos_usb_hub::{HubPort, Step};

pub const HIGH: u8 = 3;

pub fn port(connected: bool, enabled: bool, resetting: bool) -> HubPort {
    let reset_changed = enabled && !resetting;
    let (powered, speed, connect_changed) = (true, HIGH, false);
    HubPort { connected, enabled, resetting, powered, speed, connect_changed, reset_changed }
}

/// The first step at or after `from` that is not a wait, looking every ms.
pub fn next(b: &mut Bringup, st: HubPort, from: u64) -> (u64, Step) {
    (from..from + 10_000).map(|t| (t, b.step(st, t, true))).find(|&(_, s)| s != Step::Wait).unwrap()
}

pub fn next_within(b: &mut Bringup, st: HubPort, from: u64, ms: u64) -> Option<Step> {
    (from..from + ms).map(|t| b.step(st, t, true)).find(|&s| s != Step::Wait)
}
