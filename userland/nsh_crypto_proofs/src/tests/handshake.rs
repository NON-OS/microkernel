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

//! The handshake end to end, through nsh_wire's encoders and parsers: both
//! sides reach the same keys, each direction has its own, session keys are
//! new keys, and a hello changed in flight leaves the keys apart.

use super::sealer::{frame, open};
use nsh_crypto::{encapsulate, handshake_keys, session_keys, ClientKem, Keys, Secret};
use nsh_crypto::{CryptoError, Opener, Sealer, Transcript};
use nsh_wire::{ClientHello, Kind, ServerHello, CLIENT_HELLO_LEN, SERVER_HELLO_LEN};

fn same(a: &Keys, b: &Keys) -> bool {
    a.c2s.expose() == b.c2s.expose() && a.s2c.expose() == b.s2c.expose()
}

/// Both sides through the hellos, giving (client, server) keys; `tamper`
/// changes one byte of the server hello as the client receives it.
fn hellos(tamper: bool) -> ((Keys, Secret<64>), (Keys, Secret<64>)) {
    let (kem, ek) = ClientKem::new(Secret::new([0x11; 32]));
    let mut ch = [0u8; CLIENT_HELLO_LEN];
    ClientHello { random: &[1; 32], kem_ek: &ek }.encode(&mut ch).unwrap();
    let got = ClientHello::parse(&ch).unwrap();
    let (ct, s_ss) = encapsulate(got.kem_ek, Secret::new([0x22; 64])).unwrap();
    let mut sh = [0u8; SERVER_HELLO_LEN];
    ServerHello { random: &[2; 32], kem_ct: &ct }.encode(&mut sh).unwrap();
    let (mut ct_side, mut st_side) = (Transcript::default(), Transcript::default());
    st_side.push(Kind::ClientHello, &ch);
    st_side.push(Kind::ServerHello, &sh);
    if tamper {
        sh[20] ^= 1;
    }
    let seen = ServerHello::parse(&sh).unwrap();
    let c_ss = kem.decapsulate(seen.kem_ct);
    ct_side.push(Kind::ClientHello, &ch);
    ct_side.push(Kind::ServerHello, &sh);
    (handshake_keys(c_ss, &ct_side.digest()), handshake_keys(s_ss, &st_side.digest()))
}

#[test]
fn both_sides_reach_the_same_keys_and_each_direction_has_its_own() {
    let ((ck, chs), (sk, shs)) = hellos(false);
    assert!(same(&ck, &sk));
    assert_ne!(ck.c2s.expose(), ck.s2c.expose());
    let mut s2c = Sealer::new(Secret::new(*sk.s2c.expose()));
    let mut c_open = Opener::new(Secret::new(*ck.s2c.expose()));
    let proof = frame(&mut s2c, b"\x10host proof");
    assert_eq!(open(&mut c_open, &proof), Ok(b"\x10host proof".to_vec()));
    let (c_data, s_data) = (session_keys(chs, &[9; 64]), session_keys(shs, &[9; 64]));
    assert!(same(&c_data, &s_data));
    assert_ne!(c_data.c2s.expose(), ck.c2s.expose(), "session keys are new keys");
}

#[test]
fn a_server_hello_changed_in_flight_leaves_the_keys_apart() {
    let ((ck, _), (sk, _)) = hellos(true);
    assert!(!same(&ck, &sk));
    let mut s2c = Sealer::new(Secret::new(*sk.s2c.expose()));
    let mut c_open = Opener::new(Secret::new(*ck.s2c.expose()));
    let proof = frame(&mut s2c, b"\x10host proof");
    assert_eq!(open(&mut c_open, &proof), Err(CryptoError::Open));
}
