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

//! A table, a list or a grant, opened under the table and shown field by
//! field, so an operator reads what a host will act on.

use crate::error::{value, Error};
use nsh_enrol::hex::of;
use nsh_roles::{open_policy, open_revocations};
use nsh_wire::{Grant, GrantKind};

pub fn table(table: &[u8], anchor: &[u8; 32]) -> Result<String, Error> {
    let p = open_policy(table, anchor, 0)?;
    let mut s = format!("table fleet {} serial {}\n", of(p.fleet_id), p.serial);
    s += &format!("issued_ms {} revocation max age {} ms\n", p.issued_ms, p.revocation_max_age_ms);
    p.authorities.iter().for_each(|k| s += &format!("authority {}\n", of(&k.id)));
    p.approvers.iter().for_each(|k| s += &format!("approver {}\n", of(&k.id)));
    for r in &p.roles {
        let name = String::from_utf8_lossy(r.name);
        s += &format!("role {name}: {} approvals, {} ms\n", r.approvals, r.max_session_ms);
    }
    Ok(s)
}

pub fn list(table: &[u8], anchor: &[u8; 32], list: &[u8]) -> Result<String, Error> {
    let p = open_policy(table, anchor, 0)?;
    let l = open_revocations(list, &p, 0)?;
    let mut s = format!("revocations issued_ms {} count {}\n", l.list().issued_ms, l.list().len());
    l.list().ids().for_each(|id| s += &format!("revoked {}\n", of(id)));
    Ok(s)
}

pub fn grant(table: &[u8], anchor: &[u8; 32], grant: &[u8]) -> Result<String, Error> {
    let p = open_policy(table, anchor, 0)?;
    let g = Grant::parse(grant).map_err(|e| value(&format!("grant ({e:?})")))?;
    let issuer = p.authority(g.issuer).ok_or_else(|| value("issuer: not a role authority"))?;
    let digest = nsh_crypto::labelled(nsh_crypto::Label::Grant, g.signed);
    if !nsh_roles::verify(&issuer.public, &digest, &g.sig) {
        return Err(value("grant: the issuer's signature does not verify"));
    }
    let kind = if g.kind == GrantKind::Base { "base" } else { "escalation" };
    let user = String::from_utf8_lossy(g.user);
    let mut s = format!(
        "{kind} grant {} for {user}, role {}\n",
        of(g.grant_id),
        String::from_utf8_lossy(g.role)
    );
    let caps: Vec<String> =
        g.caps.iter().map(|c| String::from_utf8_lossy(c).into_owned()).collect();
    s +=
        &format!("caps {}\nfrom {} until {} ms\n", caps.join(","), g.not_before_ms, g.not_after_ms);
    s += &format!("subject {}\napprovals {}\n", of(g.subject), g.approvals().count());
    Ok(s)
}
