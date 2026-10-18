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

//! The report a finished state machine gives.

use super::counters::Counters;
use super::report::{Outcome, Report};
use crate::mlme::{Mlme, MlmeState};

pub(super) fn state_code(s: MlmeState) -> u8 {
    match s {
        MlmeState::Scanning => 0,
        MlmeState::Authenticating => 1,
        MlmeState::Associating => 2,
        MlmeState::FourWay => 3,
        MlmeState::Connected => 4,
        MlmeState::Failed => 5,
    }
}

pub(super) fn report(m: &Mlme, outcome: Outcome, counters: Counters) -> Report {
    let state = state_code(m.state());
    let (failure, akm, used_pmksa, pmksa) = (m.failure(), m.akm(), m.used_pmksa(), m.pmksa());
    let (selection, accepted, group_mgmt) = (m.selection(), m.accepted(), m.ap_group_mgmt());
    let transition_disable = m.transition_disable();
    Report {
        outcome,
        counters,
        state,
        failure,
        akm,
        used_pmksa,
        pmksa,
        selection,
        accepted,
        group_mgmt,
        transition_disable,
    }
}
