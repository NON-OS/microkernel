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

//! What the shell last read of the system: whether the network was up, the
//! clock's format and zone, and when the policy store may next be asked.

pub struct Readings {
    /// Whether the DHCP client held an address at the last tick: the menu
    /// bar's network glyph, and the edge the "network connected" toast is on.
    pub net_was_online: bool,
    pub clock_24h: bool,
    // Whole hours east of UTC, from the Timezone setting.
    pub tz_hours: i8,
    /// When the tick may next read the policy store, after one went
    /// unanswered (state/quiet_gap.rs).
    pub policy_gap: crate::state::quiet_gap::QuietGap,
}

impl Readings {
    pub fn new() -> Self {
        Readings {
            net_was_online: false,
            clock_24h: true,
            tz_hours: 0,
            policy_gap: crate::state::quiet_gap::QuietGap::new(),
        }
    }
}
