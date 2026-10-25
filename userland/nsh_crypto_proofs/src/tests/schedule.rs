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

//! The transcript chain, the key schedule, the labels and the attestation
//! challenge, each against vectors/schedule.txt, computed from the spec's
//! definitions with no code from the crate.

use crate::{arr, sched};
use nsh_crypto::Transcript;
use nsh_crypto::{handshake_keys, session_keys, Secret};
use nsh_wire::Kind;

fn frames() -> [(Kind, Vec<u8>); 5] {
    [
        (Kind::ClientHello, (0..=255u8).cycle().take(1280).collect()),
        (Kind::ServerHello, vec![0x5a; 1162]),
        (Kind::HostProof, vec![0x10; 9000]),
        (Kind::UserAuth, vec![0x20; 6000]),
        (Kind::AuthOk, vec![0x21; 40]),
    ]
}

pub fn chain() -> Vec<[u8; 64]> {
    let mut t = Transcript::default();
    let mut out = vec![t.digest()];
    for (k, b) in frames() {
        t.push(k, &b);
        out.push(t.digest());
    }
    out
}

#[test]
fn the_transcript_chain_matches() {
    let got = chain();
    assert_eq!(got[0], arr::<64>(&sched("start")));
    let text = include_str!("../../vectors/schedule.txt");
    let links: Vec<_> = text.lines().filter(|l| l.starts_with("link ")).collect();
    for (i, l) in links.iter().enumerate() {
        assert_eq!(got[i + 1], arr::<64>(l.rsplit(' ').next().unwrap()), "link {i}");
    }
    let mut t = Transcript::default();
    frames()[..2].iter().for_each(|(k, b)| t.push(*k, b));
    let pre = t.without_sigs(Kind::HostProof, &vec![0x10; 9000 - 3373]);
    assert_eq!(pre, arr::<64>(&sched("prefix")));
    assert_eq!(t.digest(), got[2], "a prefix leaves the chain as it was");
}

#[test]
fn the_key_schedule_matches() {
    let th = chain();
    let ss = Secret::new(core::array::from_fn(|i| i as u8));
    let (hk, hs) = handshake_keys(ss, &th[2]);
    assert_eq!(*hs.expose(), arr::<64>(&sched("hs")));
    assert_eq!(*hk.c2s.expose(), arr::<32>(&sched("hskey nsh/1_c2s_hs")));
    assert_eq!(*hk.s2c.expose(), arr::<32>(&sched("hskey nsh/1_s2c_hs")));
    let dk = session_keys(hs, &th[5]);
    assert_eq!(*dk.c2s.expose(), arr::<32>(&sched("datakey nsh/1_c2s_data")));
    assert_eq!(*dk.s2c.expose(), arr::<32>(&sched("datakey nsh/1_s2c_data")));
}
