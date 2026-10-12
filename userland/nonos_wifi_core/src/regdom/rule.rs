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

//! One rule of a regulatory domain, with the fields of a wireless-regdb
//! `db.txt` rule: a frequency range, the widest channel allowed in it, the
//! power ceiling, and what the station may not do there.

/// What a rule forbids.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RuleFlags(pub u8);

impl RuleFlags {
    pub const NONE: Self = Self(0);
    /// No initiating radiation: listen first, and transmit only to an access
    /// point that was heard on the channel.
    pub const NO_IR: Self = Self(1);
    /// Radar detection is required; a station only ever follows an AP here.
    pub const DFS: Self = Self(2);
    /// No OFDM: 802.11b rates only (channel 14), which this station never runs.
    pub const NO_OFDM: Self = Self(4);

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn has(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

/// A frequency range and what is allowed in it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rule {
    pub start_mhz: u32,
    pub end_mhz: u32,
    pub max_bw_mhz: u32,
    pub max_eirp_dbm: i8,
    pub flags: RuleFlags,
}

impl Rule {
    /// Whether a channel centred at `centre_mhz`, `width_mhz` wide, lies
    /// wholly inside this rule's range and width.
    pub const fn covers(&self, centre_mhz: u32, width_mhz: u32) -> bool {
        let half = width_mhz / 2;
        centre_mhz >= self.start_mhz + half
            && centre_mhz + half <= self.end_mhz
            && width_mhz <= self.max_bw_mhz
    }
}
