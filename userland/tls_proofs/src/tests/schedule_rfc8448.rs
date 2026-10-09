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

//! The in-process key schedule against RFC 8448 and against the pool.

use super::rfc8448_flight::{client, hs_keys};
use super::schedule_syscall as sys;
use super::schedule_syscall_keys::keys;
use crate::fixtures::rfc8448::{CLIENT_HELLO, SERVER_HELLO_RECORD, SERVER_MESSAGES};

/// Hash of ClientHello through server Finished, the application keys' input.
pub(super) fn th_finished() -> [u8; 32] {
    sys::hash(&[CLIENT_HELLO, &SERVER_HELLO_RECORD[5..], SERVER_MESSAGES].concat())
}

fn shared() -> [u8; 32] {
    let share = crate::server_hello::key_share(&SERVER_HELLO_RECORD[5..]).expect("share").1;
    let crate::server_share::ServerShare::X25519(peer) = share else { panic!("RFC 8448 is x25519") };
    let mut out = [0u8; 32];
    let private = client().private;
    nonos_libc::crypto_x25519_shared(private.as_ptr(), peer.as_ptr(), out.as_mut_ptr());
    out
}

#[test]
fn handshake_keys_equal_the_syscall_path_and_cost_no_round_trip() {
    let th = sys::hash(&[CLIENT_HELLO, &SERVER_HELLO_RECORD[5..]].concat());
    let secret = shared();
    let handshake = sys::next_stage(&sys::hmac(&[0; 32], &[0; 32]), &secret);
    let expected = keys(&handshake, b"hs", &th, 0x1301);
    nonos_libc::reset();
    let ours = crate::schedule::handshake_keys(&secret, &th, 0x1301).expect("keys");
    assert_eq!(nonos_libc::counts().kernel_round_trips, 0, "hashing is in-process");
    assert!(ours == expected, "the in-process schedule is the pool's schedule");
    assert!(ours == hs_keys(), "and the one HandshakeState::begin keeps");
}

#[test]
fn application_keys_equal_the_syscall_path() {
    let (hs, th) = (hs_keys(), th_finished());
    let expected = keys(&sys::next_stage(&hs.handshake_secret, &[0; 32]), b"ap", &th, 0x1301);
    nonos_libc::reset();
    let ours = crate::app_keys::app_keys(&hs, &th).expect("app keys");
    assert_eq!(nonos_libc::counts().kernel_round_trips, 0, "hashing is in-process");
    assert!(ours == expected);
}
