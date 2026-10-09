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


//! A man in the middle who relays the relay's certificate with its own key
//! share (vectors/mitm.py). It can finish the handshake, since the key share
//! is its own; only the ServerKeyExchange signature stops it, and the client
//! must stop before writing its key exchange.

use super::replay::{run, Replay};
use crate::tls12::Tls12Error;

const MITM: &[u8] = include_bytes!("../../vectors/mitm.server");

#[test]
fn a_substituted_key_share_is_refused_by_its_signature() {
    let mut r = Replay::lenient(MITM);
    let start = nonos_libc::mk_uptime_ms();
    let outcome = run(&mut r, || nonos_libc::mk_uptime_ms() - start > 20_000);
    assert_eq!(outcome, super::replay::Outcome::Refused(Tls12Error::Signature));
}
