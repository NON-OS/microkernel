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

//! The server's checks on an escalation. They run in a login's order, with
//! the user's signature the one this session already gave, and add the
//! escalation's own: one at a time, the parent is the base grant, a window
//! of at most an hour, within the base role's escalation ceiling, and the approvals
//! that role demands. The role is always the base grant's.

use crate::check::approvals::{check_approvals, MAX_ESCALATION_MS};
use crate::check::{caps, issuer, revocation, role, time, Host};
use crate::session::{grant_hash, Escalation, Login};
use crate::Refusal;
use nsh_wire::{Grant, GrantKind};

/// `active` is whether an escalation already runs in this session.
pub fn escalate(
    host: &Host<'_>,
    base: &Login,
    active: bool,
    g: &Grant<'_>,
) -> Result<Escalation, Refusal> {
    if active {
        return Err(Refusal::Active);
    }
    if g.kind != GrantKind::Escalation {
        return Err(Refusal::Kind);
    }
    issuer(host, g, Refusal::GrantSig, Refusal::GrantSig)?;
    parent(base, g)?;
    // The whole window is at most an hour. With `not_before <= now` from
    // `time`, `not_after - now` is then at most an hour too, and the grant
    // cannot be presented again days later.
    if g.not_after_ms.saturating_sub(g.not_before_ms) > MAX_ESCALATION_MS {
        return Err(Refusal::TooLong);
    }
    time(host, g)?;
    if host.now_ms >= base.ends_ms {
        return Err(Refusal::BaseEnded);
    }
    revocation(host, &[g.grant_id, g.parent_id])?;
    let role = role(host, &base.role)?;
    let caps = caps(g, role.escalation)?;
    check_approvals(host, g, role.approvals)?;
    Ok(Escalation {
        grant_id: *g.grant_id,
        caps,
        ends_ms: g.not_after_ms.min(base.ends_ms),
        hash: grant_hash(g),
    })
}

/// The parent is the base grant, and the user, subject, role and fleet are
/// the base grant's. So an escalation can never escalate an escalation.
fn parent(base: &Login, g: &Grant<'_>) -> Result<(), Refusal> {
    let same = g.parent_id == &base.grant_id
        && g.user == base.user.as_slice()
        && g.subject == &base.subject
        && g.role == base.role.as_slice()
        && g.fleet_id == &base.fleet_id;
    same.then_some(()).ok_or(Refusal::Parent)
}
