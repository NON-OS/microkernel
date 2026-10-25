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

//! A session record, `nsh-record/1`, as the host would write it: a header
//! for a handshake whose document and enrolment pass, then entries of
//! every kind but `BOX`, then a footer signed by the host keys.

use super::enrol::EnrolSpec;
pub use super::record_entry::{entry, sample};
pub use super::record_head::{header, header_with, record_doc};

pub struct Rec {
    pub header: Vec<u8>,
    pub entries: Vec<Vec<u8>>,
    pub end_ms: u64,
    pub th_hello: [u8; 64],
}

pub const SESSION: [u8; 16] = [0x5E; 16];
pub const START: u64 = 1_000;

impl Rec {
    /// A record of `n` sampled entries and an `EXIT`.
    pub fn good(n: u32) -> Rec {
        let th_hello = [0x7A; 64];
        let header = header(&th_hello, &EnrolSpec::good().bytes(), &record_doc(&th_hello));
        Rec::for_handshake(th_hello, header, n)
    }

    pub fn for_handshake(th_hello: [u8; 64], header: Vec<u8>, n: u32) -> Rec {
        let at = |s: u32| START + 10 * u64::from(s);
        let mut entries: Vec<_> = (0..n)
            .map(|s| {
                let (k, b) = sample(s);
                entry(s, at(s), k, &b)
            })
            .collect();
        entries.push(entry(n, at(n), 7, &0i32.to_le_bytes()));
        Rec { header, entries, end_ms: at(n) + 5, th_hello }
    }
}
