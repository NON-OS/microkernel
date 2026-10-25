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
//! so), D0, D1 and D2 never, and staying put never; for a claimed one,
//! D3cold alone.

use crate::device_power::{loses_config, loses_config_claimed, DState, NO_SOFT_RESET};
use crate::device_power::{needs_restore, refused};

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

/// A claimed function, whose BARs the broker puts back (L62), loses its
/// configuration only to D3cold, and is never refused a move the plain
/// rule allows.
#[test]
fn a_claimed_function_loses_it_only_to_d3cold() {
    use DState::*;
    let all = [D0, D1, D2, D3Hot, D3Cold];
    for (from, to) in all.into_iter().flat_map(|f| all.map(|t| (f, t))) {
        assert_eq!(
            loses_config_claimed(from, to),
            from != to && to == D3Cold,
            "{from:?} -> {to:?}"
        );
        for pm in [None, Some(0), Some(NO_SOFT_RESET)] {
            assert!(!loses_config_claimed(from, to) || loses_config(from, to, pm));
        }
    }
}

/// Without the kernel's restore call the plain rule holds; with it, the
/// claimed one; and the call is made after exactly the moves into D0 from
/// D3hot or D3cold.
#[test]
fn the_restore_call_decides_the_rule_and_follows_d3() {
    use DState::*;
    let all = [D0, D1, D2, D3Hot, D3Cold];
    for (from, to) in all.into_iter().flat_map(|f| all.map(|t| (f, t))) {
        for pm in [None, Some(0), Some(NO_SOFT_RESET)] {
            assert_eq!(refused(from, to, pm, false), loses_config(from, to, pm));
            assert_eq!(refused(from, to, pm, true), loses_config_claimed(from, to));
        }
        assert_eq!(needs_restore(from, to), to == D0 && from >= D3Hot, "{from:?} -> {to:?}");
    }
}
