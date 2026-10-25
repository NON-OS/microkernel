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

//! The documents' bytes, field by field.

use super::Parts;

fn sized(out: &mut Vec<u8>, v: &[u8]) {
    out.extend([&(v.len() as u32).to_be_bytes()[..], v].concat());
}

impl Parts {
    /// The common head: magic to `unconfined_grants`, then the identity
    /// count, which version 4 added.
    fn head(&self) -> Vec<u8> {
        let mut out = b"NONOSATT".to_vec();
        out.extend(self.version.to_be_bytes());
        out.extend(self.challenge);
        out.extend(self.root);
        out.extend(self.count.to_be_bytes());
        out.extend(self.flags);
        out.extend(self.grants.to_be_bytes());
        out.extend(self.identity.to_be_bytes());
        out
    }

    fn tail(&self, out: &mut Vec<u8>) {
        [&self.attest, &self.sig, &self.ak].iter().for_each(|v| sized(out, v));
    }

    pub fn v5(&self) -> Vec<u8> {
        let mut out = self.head();
        out.push(self.pcrs.len() as u8);
        for (i, v) in &self.pcrs {
            out.push(*i);
            out.extend(v);
        }
        sized(&mut out, &self.log);
        self.tail(&mut out);
        out
    }

    /// Byte for byte what `AttestationDoc::encode` writes for version 4,
    /// or wrote for version 3, which had no identity count.
    pub fn kernel(&self, version: u32) -> Vec<u8> {
        let mut out = self.head();
        if version == 3 {
            out.drain(87..91);
        }
        self.tail(&mut out);
        out
    }
}
