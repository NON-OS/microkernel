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

//! LISTEN's port and ACCEPT's listener handle off the body as net.core
//! decodes them: a body too short is E_BAD_LEN and never read past its end,
//! port 0 is E_BAD_ADDR, every other port and every handle is taken as sent.

use super::passive_body::{accept_handle, listen_port};
use crate::protocol::tcp::{E_BAD_ADDR, E_BAD_LEN};

#[test]
fn a_listen_body_shorter_than_a_port_is_bad_len() {
    assert_eq!(listen_port(&[]), Err(E_BAD_LEN));
    assert_eq!(listen_port(&[0x8e]), Err(E_BAD_LEN));
}

#[test]
fn port_zero_is_no_port() {
    assert_eq!(listen_port(&[0, 0]), Err(E_BAD_ADDR));
    assert_eq!(listen_port(&[0, 0, 0xae, 0x08]), Err(E_BAD_ADDR));
}

#[test]
fn every_other_port_is_taken_little_endian() {
    for port in 1..=u16::MAX {
        assert_eq!(listen_port(&port.to_le_bytes()), Ok(port));
    }
    assert_eq!(listen_port(&[0xae, 0x08]), Ok(2222));
}

#[test]
fn bytes_past_the_port_do_not_change_it() {
    assert_eq!(listen_port(&[0xae, 0x08, 0, 0, 0xff]), Ok(2222));
}

#[test]
fn an_accept_body_shorter_than_a_handle_is_bad_len() {
    let body = [1u8, 2, 3];
    for n in 0..body.len() {
        assert_eq!(accept_handle(&body[..n]), Err(E_BAD_LEN), "{n} bytes");
    }
}

#[test]
fn an_accept_names_the_handle_sent_whatever_follows() {
    for handle in [0, 1, 0x1000_0001, u32::MAX] {
        let mut body = handle.to_le_bytes().to_vec();
        assert_eq!(accept_handle(&body), Ok(handle));
        body.extend_from_slice(&[0xff; 9]);
        assert_eq!(accept_handle(&body), Ok(handle));
    }
}
