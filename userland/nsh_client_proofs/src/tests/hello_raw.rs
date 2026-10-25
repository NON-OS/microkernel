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

//! Plaintext frames other than a good SERVER_HELLO: an ABORT may carry
//! only VERSION or BUSY, and length and type are judged first.

use super::silent;
use crate::kit::pair::Pair;
use crate::kit::proof::ProofSpec;
use nsh_client::Refusal;
use nsh_wire::{Reason, WireError};

/// The client's answer to the good hello with `edit` applied in flight.
fn hello_edited(edit: impl FnOnce(&mut Vec<u8>)) -> (Result<(), Refusal>, Pair) {
    let mut p = Pair::good();
    let ch = p.client.take_output();
    let sh = p.server.hello(&ch, edit);
    (p.feed(&sh), p)
}

#[test]
fn a_random_or_ciphertext_changed_in_flight_leaves_nothing_to_open() {
    for at in [11, 42, 43, 600, 1162] {
        let (r, mut p) = hello_edited(|f| f[at] ^= 0x20);
        assert_eq!(r, Ok(()), "the hello itself still parses");
        let hp = p.proof(&ProofSpec::good());
        assert_eq!(p.feed(&hp), Err(Refusal::Open));
        silent(&mut p);
        assert_eq!(p.client.attester().calls.get(), 0);
    }
}

fn raw(bytes: &[u8]) -> (Result<(), Refusal>, Pair) {
    let mut p = Pair::good();
    p.client.take_output();
    (p.feed(bytes), p)
}

#[test]
fn a_plaintext_abort_carries_only_version_or_busy() {
    assert_eq!(raw(&[3, 0, 0, 0, 0x7F, 2, 0]).0, Err(Refusal::Aborted(Reason::Version)));
    assert_eq!(raw(&[3, 0, 0, 0, 0x7F, 5, 0]).0, Err(Refusal::Aborted(Reason::Busy)));
    for code in [0u8, 1, 3, 4, 16, 99] {
        let (r, mut p) = raw(&[3, 0, 0, 0, 0x7F, code, 0]);
        assert_eq!(r, Err(Refusal::Wire(WireError::Value)));
        silent(&mut p);
    }
}

#[test]
fn the_length_is_judged_before_the_body_and_the_type_before_the_hello() {
    assert_eq!(raw(&[0x01, 0x08, 0, 0]).0, Err(Refusal::Wire(WireError::Oversize)));
    assert_eq!(raw(&[0, 0, 0, 0]).0, Err(Refusal::Wire(WireError::Short)));
    assert_eq!(raw(&[1, 0, 0, 0, 0x99]).0, Err(Refusal::Wire(WireError::UnknownType)));
    assert_eq!(raw(&[1, 0, 0, 0, 0x10]).0, Err(Refusal::Wire(WireError::Order)));
    assert_eq!(raw(&[1, 0, 0, 0, 0x01]).0, Err(Refusal::Wire(WireError::Order)));
    let (_, mut p) = raw(&[1, 0, 0, 0, 0x21]);
    assert_eq!(p.feed(&[0]), Err(Refusal::Ended), "nothing is read after a refusal");
}
