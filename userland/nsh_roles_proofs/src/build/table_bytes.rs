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

//! Laying out a role table and signing it.

use super::keys::{key, K};
use super::table::{RoleB, TableB};
use super::{caps, put8, put_u16, put_u64};
use nsh_crypto::{labelled, Label};

fn keys(out: &mut Vec<u8>, list: &[K], sort: bool) {
    let mut list: Vec<_> = list.iter().map(|k| key(*k)).collect();
    if sort {
        list.sort_by_key(|k| k.id());
    }
    out.push(u8::try_from(list.len()).expect("too many keys"));
    list.iter().for_each(|k| out.extend_from_slice(&k.public()));
}

fn role(out: &mut Vec<u8>, r: &RoleB) {
    put8(out, &r.name);
    out.push(r.flags);
    out.push(r.approvals);
    put_u64(out, r.max_session_ms);
    caps(out, &r.ceiling);
    caps(out, &r.escalation);
}

impl TableB {
    /// Everything the signature covers.
    pub fn signed(&self) -> Vec<u8> {
        let mut out = b"NSHP".to_vec();
        put_u16(&mut out, 1);
        put_u16(&mut out, 0);
        out.extend_from_slice(&self.fleet);
        put_u64(&mut out, self.serial);
        put_u64(&mut out, self.issued_ms);
        put_u64(&mut out, self.max_age_ms);
        keys(&mut out, &self.authorities, self.sort_keys);
        keys(&mut out, &self.approvers, self.sort_keys);
        out.push(u8::try_from(self.roles.len()).expect("too many roles"));
        self.roles.iter().for_each(|r| role(&mut out, r));
        out.extend_from_slice(&self.signer_id);
        out
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = self.signed();
        let mut sig = key(self.signer).sign(&labelled(Label::Policy, &out));
        self.spoil.apply(&mut sig);
        out.extend_from_slice(&sig);
        out
    }
}
