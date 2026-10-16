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

//! The moves a PCI function's configuration does not survive, over every
//! pair of states and every power management status: D3cold always, D3hot
//! unless No_Soft_Reset is set (and never without a capability to say
//! so), D0, D1 and D2 never, and staying put never.

use crate::device_power::{loses_config, DState, NO_SOFT_RESET};

#[test]
fn every_move_and_every_status() {
    use DState::*;
    let all = [D0, D1, D2, D3Hot, D3Cold];
    let statuses = [None, Some(0), Some(NO_SOFT_RESET), Some(0xFFFF), Some(!NO_SOFT_RESET)];
    for (from, to) in all.into_iter().flat_map(|f| all.map(|t| (f, t))) {
        for pm in statuses {
            let nsr = pm.is_some_and(|p| p & NO_SOFT_RESET != 0);
            let want = from != to && (to == D3Cold || (to == D3Hot && !nsr));
            assert_eq!(loses_config(from, to, pm), want, "{from:?} -> {to:?}, {pm:?}");
        }
    }
}
