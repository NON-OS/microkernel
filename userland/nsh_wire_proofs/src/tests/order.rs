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

//! Which frame may arrive when. The admitted set of every stage is written
//! out here from docs/design/nsh-v1-wire*.md, and the parser's set must be
//! that one exactly: nothing missing, nothing extra.

use super::stages::ALL;
use nsh_wire::{split, Expect, Kind, WireError};
use Kind::*;

fn spec(e: Expect) -> &'static [Kind] {
    match e {
        Expect::ClientHello => &[ClientHello],
        Expect::ServerHello => &[ServerHello, Abort],
        Expect::HostProof => &[HostProof, Abort],
        Expect::UserAuth => &[UserAuth, Abort],
        Expect::AuthResult => &[AuthOk, AuthRefused, Abort],
        Expect::ServerSession => &[Input, Window, Signal, Escalate, Close, Abort],
        Expect::ClientSession => &[Output, EscalateResult, Exit, Record, Close, Abort],
    }
}

#[test]
fn every_stage_admits_exactly_the_specified_kinds() {
    for e in ALL {
        for b in 0..=255u8 {
            let one = [b];
            let got = split(&one, e);
            match Kind::from_byte(b) {
                Err(_) => assert_eq!(got, Err(WireError::UnknownType), "{e:?} {b:#x}"),
                Ok(k) if spec(e).contains(&k) => assert_eq!(got, Ok((k, &[][..])), "{e:?} {k:?}"),
                Ok(k) => assert_eq!(got, Err(WireError::Order), "{e:?} {k:?}"),
            }
        }
    }
}

#[test]
fn exactly_sixteen_kinds_are_defined_and_each_byte_round_trips() {
    let known: Vec<u8> = (0..=255u8).filter(|b| Kind::from_byte(*b).is_ok()).collect();
    assert_eq!(
        known,
        [1, 2, 0x10, 0x20, 0x21, 0x22, 0x30, 0x31, 0x32, 0x33, 0x40, 0x41, 0x50, 0x51, 0x52, 0x7F]
    );
    known.iter().for_each(|b| assert_eq!(Kind::from_byte(*b).map(Kind::byte), Ok(*b)));
}

#[test]
fn no_frame_after_login_is_admitted_before_it() {
    for k in [Input, Output, Window, Signal, Escalate, EscalateResult, Exit, Record, Close] {
        for e in [Expect::ClientHello, Expect::ServerHello, Expect::HostProof, Expect::UserAuth] {
            assert_eq!(split(&[k.byte()], e), Err(WireError::Order), "{k:?} at {e:?}");
        }
    }
}
