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

//! The hellos: exact lengths, the preamble judged first, and every byte of
//! the preamble meaningful.

use nsh_wire::{ClientHello, ServerHello, WireError, CLIENT_HELLO_LEN, SERVER_HELLO_LEN};

pub fn client() -> Vec<u8> {
    let mut out = vec![0u8; CLIENT_HELLO_LEN];
    let n = ClientHello { random: &[0x52; 32], kem_ek: &[0xEE; 1216] }.encode(&mut out);
    assert_eq!(n, Ok(CLIENT_HELLO_LEN));
    out
}

#[test]
fn the_lengths_are_the_specified_ones() {
    assert_eq!((CLIENT_HELLO_LEN, SERVER_HELLO_LEN), (1258, 1162));
}

#[test]
fn a_client_hello_round_trips() {
    let bytes = client();
    assert_eq!(&bytes[..10], b"NSH1\x02\x00\x01\x00\x00\x00");
    let h = ClientHello::parse(&bytes).expect("valid");
    assert_eq!((h.random, &h.kem_ek[..]), (&[0x52; 32], &[0xEE; 1216][..]));
}

#[test]
fn a_server_hello_round_trips() {
    let mut out = vec![0u8; SERVER_HELLO_LEN];
    let sent = ServerHello { random: &[1; 32], kem_ct: &[2; 1120] };
    assert_eq!(sent.encode(&mut out), Ok(SERVER_HELLO_LEN));
    assert_eq!(ServerHello::parse(&out), Ok(sent));
    assert_eq!(ClientHello::parse(&out), Err(WireError::Short));
}

#[test]
fn one_byte_more_or_less_is_refused() {
    let mut bytes = client();
    assert_eq!(ClientHello::parse(&bytes[..CLIENT_HELLO_LEN - 1]), Err(WireError::Short));
    bytes.push(0);
    assert_eq!(ClientHello::parse(&bytes), Err(WireError::Trailing));
}
