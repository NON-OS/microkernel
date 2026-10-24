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

//! Laying a grant out, and the escalation variant.

use super::grant::GrantB;
use super::{caps, put8, put_u16, put_u64, sig};

impl GrantB {
    /// A valid escalation under the default base grant, approved by one.
    pub fn escalation() -> Self {
        let base = GrantB::default();
        GrantB {
            kind: 2,
            grant_id: [0x22; 16],
            parent_id: base.grant_id,
            approvers: vec![[0xA1; 32]],
            ..base
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = self.magic.to_vec();
        put_u16(&mut out, self.version);
        out.extend_from_slice(&[self.kind, self.reserved]);
        [&self.grant_id[..], &self.parent_id, &self.fleet_id, &self.subject]
            .iter()
            .for_each(|f| out.extend_from_slice(f));
        put8(&mut out, &self.user);
        put8(&mut out, &self.role);
        caps(&mut out, &self.caps.iter().map(Vec::as_slice).collect::<Vec<_>>());
        put_u64(&mut out, self.not_before);
        put_u64(&mut out, self.not_after);
        out.extend_from_slice(&self.issuer);
        sig(&mut out, 0xE5);
        out.push(u8::try_from(self.approvers.len()).expect("approvals"));
        for a in &self.approvers {
            out.extend_from_slice(a);
            sig(&mut out, 0xA5);
        }
        out
    }
}
