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

//! `nsh-policy/1`, the role table, laid out field by field
//! (docs/nsh/nsh-v1-roles.md), signed by a role authority, and opened as a
//! host opens it under that authority's key id before it is handed back.

use crate::error::{value, Error};
use crate::keys::{id, sorted, Public};
use crate::put;
use crate::roles_file::Role;
use nsh_crypto::{labelled, Label};
use nsh_enrol::AuthorityKey;
use nsh_roles::{open_policy, POLICY_MAGIC};

pub struct TableSpec {
    pub fleet: [u8; 16],
    pub serial: u64,
    pub issued_ms: u64,
    pub max_age_ms: u64,
    pub authorities: Vec<Public>,
    pub approvers: Vec<Public>,
    pub roles: Vec<Role>,
}

fn keys(out: &mut Vec<u8>, keys: Vec<Public>, what: &str) -> Result<(), Error> {
    let keys = sorted(keys)?;
    out.push(u8::try_from(keys.len()).map_err(|_| value(what))?);
    keys.iter().for_each(|k| out.extend_from_slice(&k[..]));
    Ok(())
}

fn role(out: &mut Vec<u8>, r: &Role) -> Result<(), Error> {
    put::bytes8(out, r.name.as_bytes(), "role name")?;
    out.push(u8::from(r.digest));
    out.push(r.approvals);
    put::u64(out, r.max_session_ms);
    put::caps(out, &r.ceiling)?;
    put::caps(out, &r.escalation)
}

pub fn table(mut spec: TableSpec, signer: &AuthorityKey) -> Result<Vec<u8>, Error> {
    let mut out = POLICY_MAGIC.to_vec();
    put::u16(&mut out, 1);
    put::u16(&mut out, 0);
    out.extend_from_slice(&spec.fleet);
    put::u64(&mut out, spec.serial);
    put::u64(&mut out, spec.issued_ms);
    put::u64(&mut out, spec.max_age_ms);
    keys(&mut out, spec.authorities, "authority count")?;
    keys(&mut out, spec.approvers, "approver count")?;
    spec.roles.sort_by(|a, b| a.name.cmp(&b.name));
    out.push(u8::try_from(spec.roles.len()).map_err(|_| value("role count"))?);
    spec.roles.iter().try_for_each(|r| role(&mut out, r))?;
    out.extend_from_slice(&id(&signer.public()));
    let sig = signer.sign(&labelled(Label::Policy, &out))?;
    out.extend_from_slice(&sig);
    open_policy(&out, &signer.id(), 0)?;
    Ok(out)
}
