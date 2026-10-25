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

//! Laying a record out.

use super::enrol::EnrolB;
use super::record::RecordB;
use super::{caps, public, put16, put8, put_u16, put_u32, put_u64, sig};

impl RecordB {
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = b"NSHR".to_vec();
        put_u16(&mut out, 1);
        put_u16(&mut out, 0);
        out.extend_from_slice(&[0x5E; 16]);
        put_u64(&mut out, self.start);
        out.extend_from_slice(&[0x7A; 64]);
        public(&mut out, 0x4B);
        put16(&mut out, &EnrolB::default().bytes());
        [[0xD0; 64], [0x5B; 64]].iter().for_each(|d| out.extend_from_slice(d));
        put8(&mut out, b"ek");
        put8(&mut out, b"auditor");
        out.extend_from_slice(&[0x6A; 64]);
        caps(&mut out, &self.caps.iter().map(Vec::as_slice).collect::<Vec<_>>());
        for e in &self.entries {
            put_u32(&mut out, e.seq);
            put_u64(&mut out, e.at);
            out.push(e.kind);
            put16(&mut out, &e.body);
        }
        let count = self.count.unwrap_or(self.entries.len() as u32);
        put_u32(&mut out, count);
        put_u64(&mut out, self.end);
        out.push(self.end_reason);
        out.extend_from_slice(&[0xC4; 64]);
        sig(&mut out, 0x5D);
        out
    }
}
