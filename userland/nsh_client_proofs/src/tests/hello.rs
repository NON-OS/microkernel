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

//! `SERVER_HELLO`, the plaintext frame. Each run changes one thing about
//! the good hello. Before keys exist the client can tell the server
//! nothing, so every refusal here sends nothing; a hello changed in flight
//! leaves the keys apart, and the first sealed frame does not open.

use super::silent;
use crate::kit::pair::Pair;
use nsh_client::Refusal;
use nsh_wire::WireError;

/// The client's answer to the good hello with `edit` applied in flight.
fn hello_edited(edit: impl FnOnce(&mut Vec<u8>)) -> (Result<(), Refusal>, Pair) {
    let mut p = Pair::good();
    let ch = p.client.take_output();
    let sh = p.server.hello(&ch, edit);
    (p.feed(&sh), p)
}

fn refused(edit: impl FnOnce(&mut Vec<u8>), want: WireError) {
    let (r, mut p) = hello_edited(edit);
    assert_eq!(r, Err(Refusal::Wire(want)));
    silent(&mut p);
}

#[test]
fn a_changed_preamble_is_refused_for_what_it_breaks() {
    refused(|f| f[1] ^= 1, WireError::Magic);
    refused(|f| f[5] = 1, WireError::Version);
    refused(|f| f[7] = 2, WireError::Version);
    refused(|f| f[9] = 1, WireError::Reserved);
    refused(|f| f[10] = 1, WireError::Reserved);
}

#[test]
fn a_short_or_long_hello_is_refused() {
    refused(
        |f| {
            f.pop();
        },
        WireError::Short,
    );
    refused(|f| f.push(0), WireError::Trailing);
}
