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

//! What every grant that parses must be, as the specification states it.

use nsh_wire::{names, Grant, GrantKind};

pub fn hold(g: &Grant<'_>) {
    assert!(g.grant_id.iter().any(|b| *b != 0));
    assert!(g.fleet_id.iter().any(|b| *b != 0));
    assert!(names::user(g.user).is_ok() && names::role(g.role).is_ok());
    assert!(!g.caps.is_empty() && g.caps.len() <= nsh_wire::MAX_CAPS);
    assert!(g.not_before_ms < g.not_after_ms);
    let ids: Vec<_> = g.approvals().map(|a| *a.approver).collect();
    assert!(ids.len() <= 2);
    assert!(ids.iter().all(|a| a != g.issuer));
    if let [a, b] = ids[..] {
        assert_ne!(a, b);
    }
    match g.kind {
        GrantKind::Base => {
            assert!(g.parent_id.iter().all(|b| *b == 0));
            assert!(ids.is_empty());
        }
        GrantKind::Escalation => {
            assert!(g.parent_id.iter().any(|b| *b != 0));
            assert_ne!(g.parent_id, g.grant_id);
        }
    }
}
