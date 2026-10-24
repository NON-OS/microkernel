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

//! Every parser is total: on bytes that are noise, or a valid body with
//! one byte changed or cut short, it returns an answer and never panics.
//! A seeded generator keeps every run the same; the fuzz targets go wider.

use crate::build::{grant::GrantB, proof::HostProofB, record::RecordB};
use nsh_wire::{split, AuthOk, ClientHello, Enrol, Expect, Grant, HostProof, Record, UserAuth};

fn every_parser(b: &[u8]) {
    let _ = ClientHello::parse(b);
    let _ = AuthOk::parse(b);
    let _ = Enrol::parse(b);
    let _ = Grant::parse(b);
    let _ = HostProof::parse(b);
    let _ = UserAuth::parse(b);
    let _ = nsh_wire::escalate(b);
    if let Ok(r) = Record::parse(b) {
        assert_eq!(r.entries().count() as u32, r.footer.count);
    }
    for e in super::stages::ALL {
        let _ = split(b, e);
    }
    let _ = split(b, Expect::ServerSession);
}

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

#[test]
fn noise_never_panics() {
    let mut g = Lcg(0x6E73_6831);
    for _ in 0..2_000 {
        let n = (g.next() % 600) as usize;
        let b: Vec<u8> = (0..n).map(|_| g.next() as u8).collect();
        every_parser(&b);
    }
}

#[test]
fn each_valid_body_with_one_byte_changed_or_cut_never_panics() {
    let mut g = Lcg(7);
    let bodies =
        [GrantB::escalation().bytes(), HostProofB::default().bytes(), RecordB::default().bytes()];
    for body in &bodies {
        for _ in 0..300 {
            let mut b = body.clone();
            let i = (g.next() as usize) % b.len();
            b[i] = g.next() as u8;
            every_parser(&b);
            every_parser(&b[..i]);
        }
    }
}
