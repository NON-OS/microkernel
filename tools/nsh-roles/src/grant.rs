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

//! `nsh-grant/1`: a base grant, or an escalation of one, issued by a role
//! authority of the table to one user's key, with no approvals yet. The
//! grant is read back with `nsh_wire`'s parser and its issuer checked under
//! the table before it is handed back.

use crate::error::{value, Error};
use crate::put;
use nsh_crypto::{labelled, Label};
use nsh_enrol::AuthorityKey;
use nsh_roles::open_policy;
use nsh_wire::grant::GRANT_MAGIC;
use nsh_wire::{Grant, GrantKind};

pub struct GrantSpec {
    pub kind: GrantKind,
    pub grant_id: [u8; 16],
    /// The base grant's id for an escalation; zeros for a base grant.
    pub parent_id: [u8; 16],
    pub subject: [u8; 64],
    pub user: String,
    pub role: String,
    pub caps: Vec<String>,
    pub not_before_ms: u64,
    pub not_after_ms: u64,
}

pub fn grant(
    table: &[u8],
    anchor: &[u8; 32],
    g: &GrantSpec,
    k: &AuthorityKey,
) -> Result<Vec<u8>, Error> {
    let policy = open_policy(table, anchor, 0)?;
    let mut out = GRANT_MAGIC.to_vec();
    put::u16(&mut out, 1);
    out.extend_from_slice(&[g.kind as u8, 0]);
    out.extend_from_slice(&g.grant_id);
    out.extend_from_slice(&g.parent_id);
    out.extend_from_slice(policy.fleet_id);
    out.extend_from_slice(&g.subject);
    put::bytes8(&mut out, g.user.as_bytes(), "user name")?;
    put::bytes8(&mut out, g.role.as_bytes(), "role name")?;
    put::caps(&mut out, &g.caps)?;
    put::u64(&mut out, g.not_before_ms);
    put::u64(&mut out, g.not_after_ms);
    out.extend_from_slice(&k.id());
    out.extend_from_slice(&k.sign(&labelled(Label::Grant, &out))?);
    out.push(0);
    let parsed = Grant::parse(&out).map_err(|e| value(&format!("grant ({e:?})")))?;
    if policy.authority(parsed.issuer).is_none() {
        return Err(value("issuer: the key is not a role authority of the table"));
    }
    if policy.role(parsed.role).is_none() {
        return Err(value("role: the table does not name it"));
    }
    Ok(out)
}
