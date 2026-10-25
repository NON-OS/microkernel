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

//! The server's checks on a login, in the order roles.md gives. The first
//! part proves the key: the issuer, the subject and the user's signature.
//! Nothing about the grant, the role or the list is judged before that
//! part passes, so nobody learns of them without holding the user's key.

use crate::check::{caps, issuer, revocation, role, time, Host};
use crate::hybrid::verify;
use crate::session::{grant_hash, Login};
use crate::Refusal;
use nsh_crypto::{key_digest, labelled, Label};
use nsh_wire::{GrantKind, UserAuth};

/// `th` is `TH(USER_AUTH without its signatures)`, from the transcript.
pub fn login(host: &Host<'_>, auth: &UserAuth<'_>, th: &[u8; 64]) -> Result<Login, Refusal> {
    let g = &auth.grant;
    if g.kind != GrantKind::Base {
        return Err(Refusal::Kind);
    }
    issuer(host, g, Refusal::Issuer, Refusal::IssuerSig)?;
    if g.subject != &key_digest(auth.user.ed, auth.user.ml) {
        return Err(Refusal::Subject);
    }
    if !verify(&auth.user, &labelled(Label::UserSig, th), &auth.sig) {
        return Err(Refusal::UserSig);
    }
    if g.fleet_id != host.policy.fleet_id {
        return Err(Refusal::Fleet);
    }
    time(host, g)?;
    revocation(host, &[g.grant_id])?;
    let role = role(host, g.role)?;
    let caps = caps(g, role.ceiling)?;
    let session_end = host.now_ms.saturating_add(role.max_session_ms);
    Ok(Login {
        grant_id: *g.grant_id,
        fleet_id: *g.fleet_id,
        subject: *g.subject,
        user: g.user.into(),
        role: g.role.into(),
        caps,
        ends_ms: g.not_after_ms.min(session_end),
        hash: grant_hash(g),
        record_digest: role.record_digest,
    })
}
