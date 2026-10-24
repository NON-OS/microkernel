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

//! The default escalation grant: under the base grant, adding `Network`
//! for half an hour, approved by one approver.

use super::grant::{ApprovalB, GrantB};
use super::keys::K;
use super::{BASE_ID, ESC_ID, HOUR, NOW};

impl GrantB {
    pub fn escalation() -> Self {
        GrantB {
            kind: 2,
            grant_id: ESC_ID,
            parent_id: BASE_ID,
            caps: vec!["CoreExec", "IPC", "Memory", "Network"],
            not_before_ms: NOW - 60_000,
            not_after_ms: NOW + HOUR / 2,
            approvals: vec![ApprovalB::by(K::Appr1)],
            ..GrantB::default()
        }
    }
}
