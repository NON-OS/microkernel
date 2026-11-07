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

//! `CLIENT_HELLO` and the host's proof: only `VERSION` and `BUSY` can be
//! told before keys exist; everything else broken there is closed without
//! a word. A host that cannot attest or sign says `BUSY` or nothing, and
//! never sends a proof the client would refuse.

use crate::kit::raw_run::RawRun;
use nsh_server::Why;
use nsh_wire::WireError;

pub(super) const VERSION_ABORT: [u8; 7] = [3, 0, 0, 0, 0x7F, 2, 0];

#[test]
fn another_version_is_told_version_in_plaintext() {
    let mut r = RawRun::new();
    let mut hello = r.raw.hello.clone();
    hello[9] = 9;
    assert_eq!(r.send(&hello), VERSION_ABORT);
    assert_eq!(r.refused(), [Why::Wire(WireError::Version)]);
    assert!(r.server.is_over());
    let mut r = RawRun::new();
    let mut hello = r.raw.hello.clone();
    hello[11] = 2;
    assert_eq!(r.send(&hello), VERSION_ABORT, "another suite");
}

#[test]
fn a_broken_hello_is_closed_without_a_word() {
    for (at, v) in [(5, b'X'), (13, 1), (14, 1)] {
        let mut r = RawRun::new();
        let mut hello = r.raw.hello.clone();
        hello[at] = v;
        assert!(r.send(&hello).is_empty(), "byte {at}");
        assert!(r.server.is_over());
    }
    let mut r = RawRun::new();
    let mut short = r.raw.hello.clone();
    short.pop();
    short[0] -= 1;
    assert!(r.send(&short).is_empty());
    assert_eq!(r.refused(), [Why::Wire(WireError::Short)]);
}

#[test]
fn a_length_over_the_hello_cap_is_refused_before_its_body() {
    let mut r = RawRun::new();
    assert!(r.send(&2049u32.to_le_bytes()).is_empty());
    assert_eq!(r.refused(), [Why::Wire(WireError::Oversize)]);
    let mut r = RawRun::new();
    assert!(r.send(&[0, 0, 0, 0]).is_empty());
    assert_eq!(r.refused(), [Why::Wire(WireError::Short)]);
}
