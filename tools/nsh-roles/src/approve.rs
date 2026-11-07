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

//! An approval added to an escalation grant: the approver's key id and
//! hybrid signature over `H("nsh/1 approve\0" || H("nsh/1 grant\0" ||
//! signed bytes))`. The approver must be one the table lists, neither the
//! issuer nor the grant's subject, and not one who has approved already.
//! The approvals are kept in ascending approver order, and every one is
//! checked again under the table before the grant is handed back.

use crate::error::{value, Error};
use nsh_crypto::{labelled, Label};
use nsh_enrol::AuthorityKey;
use nsh_roles::{approvals, open_policy};
use nsh_wire::{Approval, Grant, GrantKind, MAX_APPROVALS};

pub fn approve(
    table: &[u8],
    anchor: &[u8; 32],
    grant: &[u8],
    k: &AuthorityKey,
) -> Result<Vec<u8>, Error> {
    let policy = open_policy(table, anchor, 0)?;
    let g = Grant::parse(grant).map_err(|e| value(&format!("grant ({e:?})")))?;
    if g.kind != GrantKind::Escalation {
        return Err(value("grant: only an escalation carries approvals"));
    }
    let me = k.id();
    let mut list: Vec<([u8; 32], Vec<u8>)> =
        g.approvals().map(|a| (*a.approver, [&a.sig.ed[..], &a.sig.ml[..]].concat())).collect();
    if list.iter().any(|(id, _)| *id == me) {
        return Err(value("approver: this key has approved the grant already"));
    }
    if list.len() == MAX_APPROVALS {
        return Err(value("approvals: the grant has as many as any role demands"));
    }
    let digest = labelled(Label::Approve, &labelled(Label::Grant, g.signed));
    list.push((me, k.sign(&digest)?));
    list.sort_by_key(|(id, _)| *id);
    let mut out = g.issued.to_vec();
    out.push(list.len() as u8);
    list.iter().for_each(|(id, sig)| out.extend_from_slice(&[&id[..], sig].concat()));
    let made = Grant::parse(&out).map_err(|e| value(&format!("approved grant ({e:?})")))?;
    let given: Vec<Approval<'_>> = made.approvals().collect();
    let n = given.len() as u8;
    approvals(&policy, made.subject, made.issuer, &digest, &given, n).map_err(Error::Grant)?;
    Ok(out)
}
