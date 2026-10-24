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

//! `NONOSATT` documents, version 4 as k1-attest-doc-v4.md lays it out and
//! version 3 as the kernel encodes it today.

#[derive(Clone)]
pub struct Parts {
    pub version: u32,
    pub challenge: [u8; 32],
    pub root: [u8; 32],
    pub count: u32,
    pub flags: [u8; 3],
    pub grants: u32,
    pub pcrs: Vec<(u8, [u8; 32])>,
    pub log: Vec<u8>,
    pub attest: Vec<u8>,
    pub sig: Vec<u8>,
    pub ak: Vec<u8>,
}

fn sized(out: &mut Vec<u8>, v: &[u8]) {
    out.extend([&(v.len() as u32).to_be_bytes()[..], v].concat());
}

impl Parts {
    /// The common head: magic to `unconfined_grants`.
    fn head(&self) -> Vec<u8> {
        let mut out = b"NONOSATT".to_vec();
        out.extend(self.version.to_be_bytes());
        out.extend(self.challenge);
        out.extend(self.root);
        out.extend(self.count.to_be_bytes());
        out.extend(self.flags);
        out.extend(self.grants.to_be_bytes());
        out
    }

    fn tail(&self, out: &mut Vec<u8>) {
        [&self.attest, &self.sig, &self.ak].iter().for_each(|v| sized(out, v));
    }

    pub fn v4(&self) -> Vec<u8> {
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

    /// Byte for byte what `AttestationDoc::encode` writes for version 3.
    pub fn v3(&self) -> Vec<u8> {
        let mut out = self.head();
        self.tail(&mut out);
        out
    }
}
