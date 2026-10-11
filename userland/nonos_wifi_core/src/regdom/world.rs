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

//! The world regulatory domain, wireless-regdb's `country 00`: what is legal
//! to receive everywhere and to transmit nowhere it is not. It is the default
//! until the owner chooses a country (a letter decision), so a station that
//! does not know where it is never initiates on a channel some country keeps
//! for radar or for other services. 6 GHz is closed in it.

use super::domain::Domain;
use super::rule::{Rule, RuleFlags as F};

const fn rule(start: u32, end: u32, bw: u32, eirp: i8, flags: F) -> Rule {
    Rule { start_mhz: start, end_mhz: end, max_bw_mhz: bw, max_eirp_dbm: eirp, flags }
}

const NO_IR_DFS: F = F::NO_IR.union(F::DFS);

static WORLD_RULES: [Rule; 7] = [
    // Channels 1 to 11, open everywhere.
    rule(2402, 2472, 40, 20, F::NONE),
    // Channels 12 and 13: only where an AP is heard.
    rule(2457, 2482, 20, 20, F::NO_IR),
    // Channel 14: Japan only, and 802.11b only.
    rule(2474, 2494, 20, 20, F::NO_IR.union(F::NO_OFDM)),
    // Channels 36 to 48.
    rule(5170, 5250, 80, 20, F::NO_IR),
    // Channels 52 to 64 and 100 to 144 share their band with radar.
    rule(5250, 5330, 80, 20, NO_IR_DFS),
    rule(5490, 5730, 160, 20, NO_IR_DFS),
    // Channels 149 to 165.
    rule(5735, 5835, 80, 20, F::NO_IR),
];

/// The world domain.
pub const WORLD: Domain = Domain::new(*b"00", &WORLD_RULES);
