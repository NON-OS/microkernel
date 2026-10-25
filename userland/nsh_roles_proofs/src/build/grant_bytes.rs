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

//! Laying out a grant and signing it: the issuer over
//! `H("nsh/1 grant\0" || fields)`, each approver over
//! `H("nsh/1 approve\0" || that digest)`.

use super::grant::GrantB;
use super::keys::key;
use super::{caps, put8, put_u16, put_u64};
use nsh_crypto::{labelled, Label};

impl GrantB {
    pub fn signed(&self) -> Vec<u8> {
        let mut out = b"NSHG".to_vec();
        put_u16(&mut out, 1);
        out.extend_from_slice(&[self.kind, 0]);
        out.extend_from_slice(&self.grant_id);
        out.extend_from_slice(&self.parent_id);
        out.extend_from_slice(&self.fleet);
        out.extend_from_slice(&self.subject);
        put8(&mut out, &self.user);
        put8(&mut out, &self.role);
        caps(&mut out, &self.caps);
        put_u64(&mut out, self.not_before_ms);
        put_u64(&mut out, self.not_after_ms);
        out.extend_from_slice(&self.issuer);
        out
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = self.signed();
        let digest = labelled(Label::Grant, &out);
        let mut sig = key(self.signer).sign(&digest);
        self.spoil.apply(&mut sig);
        out.extend_from_slice(&sig);
        let mut approvals = self.approvals.clone();
        approvals.sort_by_key(|a| a.id);
        out.push(u8::try_from(approvals.len()).expect("too many approvals"));
        for a in approvals {
            let over = match a.unlabelled {
                true => digest,
                false => labelled(Label::Approve, &digest),
            };
            let mut sig = key(a.signer).sign(&over);
            a.spoil.apply(&mut sig);
            out.extend_from_slice(&a.id);
            out.extend_from_slice(&sig);
        }
        out
    }
}
