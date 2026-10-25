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

//! The length field is judged before the body: at each stage's cap it
//! passes, one over it is `Oversize`, and a sealed frame too short to hold
//! a type byte and a tag is `Short`.

use super::stages::ALL;
use nsh_wire::{split, wire_len, Expect, Kind, WireError};
use nsh_wire::{AUTH_CAP, HELLO_CAP, SESSION_CAP, TAG_LEN};

const _: () = assert!(nsh_wire::CLIENT_HELLO_LEN < HELLO_CAP);

fn len(n: u32, e: Expect) -> Result<usize, WireError> {
    wire_len(n.to_le_bytes(), e)
}

#[test]
fn each_stage_takes_its_cap_and_refuses_one_more() {
    for e in ALL {
        let tag = if e.sealed() { TAG_LEN } else { 0 };
        let max = (e.cap() + tag) as u32;
        assert_eq!(len(max, e), Ok(max as usize), "{e:?}");
        assert_eq!(len(max + 1, e), Err(WireError::Oversize), "{e:?}");
        assert_eq!(len(u32::MAX, e), Err(WireError::Oversize), "{e:?}");
    }
}

#[test]
fn the_caps_are_the_specified_ones() {
    assert_eq!((HELLO_CAP, AUTH_CAP, SESSION_CAP), (2048, 262_144, 16_385));
    assert_eq!(Expect::ClientHello.cap(), HELLO_CAP);
    assert_eq!(Expect::UserAuth.cap(), AUTH_CAP);
    assert_eq!(Expect::ClientSession.cap(), SESSION_CAP);
}

#[test]
fn only_the_hellos_are_plaintext() {
    let plain: Vec<_> = ALL.iter().filter(|e| !e.sealed()).collect();
    assert_eq!(plain, [&Expect::ClientHello, &Expect::ServerHello]);
}

#[test]
fn a_frame_must_hold_a_type_and_for_sealed_frames_a_tag() {
    assert_eq!(len(0, Expect::ClientHello), Err(WireError::Short));
    assert_eq!(len(1, Expect::ClientHello), Ok(1));
    assert_eq!(len(16, Expect::UserAuth), Err(WireError::Short));
    assert_eq!(len(17, Expect::UserAuth), Ok(17));
}

#[test]
fn split_refuses_an_inner_frame_over_the_cap_and_an_empty_one() {
    let big = vec![Kind::Input.byte(); SESSION_CAP + 1];
    assert_eq!(split(&big, Expect::ServerSession), Err(WireError::Oversize));
    assert_eq!(split(&big[..SESSION_CAP], Expect::ServerSession).map(|f| f.0), Ok(Kind::Input));
    assert_eq!(split(&[], Expect::ServerSession), Err(WireError::Short));
}
