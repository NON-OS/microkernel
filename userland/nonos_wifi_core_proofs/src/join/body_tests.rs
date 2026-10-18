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

//! Hostile connect bodies from the shell: a malformed body is refused with a
//! well-formed reply and never a panic, and a body carrying a secret
//! passphrase never leaks it into a transmitted frame, whatever its flags.

use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::rig::{rig, Station};
use crate::soft::wire::{code, control};
use crate::sweep::rng::Rng;

const SSID: &[u8] = b"ThisIsASSID";
const SECRET: &[u8] = b"SECRETpassphrase";

fn ap() -> Station {
    rig(SimAp::new(SSID, b"ThisIsAPassword", 6, &RSNE_PSK))
}

fn body(ssid: &[u8], pass: &[u8], flags: u8) -> Vec<u8> {
    let mut b = vec![ssid.len() as u8];
    b.extend_from_slice(ssid);
    b.push(pass.len() as u8);
    b.extend_from_slice(pass);
    b.push(flags);
    b
}

#[test]
fn malformed_bodies_are_refused_cleanly() {
    let cases: [&[u8]; 6] = [
        &[],
        &[0],                              // ssid_len 0
        &[33, 1, 2, 3],                    // ssid_len past the body
        &[4, b'n', b'e', b't'],            // ssid runs off, no pass_len
        &[4, b'n', b'e', b't', b'w', 200], // pass_len past the body
        &[11, b'T', b'h', b'i', b's', b'I', b's', b'A', b'S', b'S', b'I', b'D', 0, 0xFF],
    ];
    for c in cases {
        let mut st = ap();
        let r = control(&mut st, 1, c);
        assert!(r.len() >= 4 && code(&r) <= 0, "a hostile body is refused with a status");
    }
}

#[test]
fn a_secret_passphrase_never_leaves() {
    let names: [&[u8]; 3] = [SSID, b"other", b""];
    let mut rng = Rng::new(0x5057_4150);
    for _ in 0..240 {
        let ssid = rng.pick(&names);
        let mut st = ap();
        let _ = control(&mut st, 1, &body(ssid, SECRET, rng.byte()));
        let air = st.radio().unwrap();
        let leaked = air.sent.iter().any(|f| f.windows(SECRET.len()).any(|w| w == SECRET));
        assert!(!leaked, "the passphrase was transmitted");
    }
}
