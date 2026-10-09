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

//! A framed response ends in the call that brings its last byte.

use super::fetch_fixtures::{step, url_of};
use super::fetch_tls_fixtures::settled;
use super::fetch_wire::FakeWire;
use crate::browser::fetch::run::run;
use crate::browser::fetch::types::{Fetch, Phase};

pub const SIZED: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\n0123456789";
pub const CHUNKED: &[u8] =
    b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n4\r\n0123\r\n6\r\n456789\r\n0\r\n\r\n";

pub fn reading(scheme: &str) -> Fetch {
    Fetch::new(url_of(&format!("{scheme}://10.0.2.2/")), 11, Phase::ReadBody, 0)
}

/// Deliver `wire` in two reads cut at every offset: the fetch reads on
/// after the first and ends in the call that brings the second.
pub fn ends_at_last_byte(wire: &[u8], scheme: &str, end: Phase) {
    for cut in 1..wire.len() {
        let mut w = FakeWire::at(0);
        let mut f = reading(scheme);
        if scheme == "https" {
            f.tls = Some(settled());
        }
        w.deliver(11, &wire[..cut]);
        run(&mut w, &mut f, 30);
        assert_eq!(f.phase, Phase::ReadBody, "{scheme} cut at {cut}");
        w.deliver(11, &wire[cut..]);
        run(&mut w, &mut f, 30);
        assert_eq!(f.phase, end, "{scheme} cut at {cut}: no close needed");
    }
}

#[test]
fn a_sized_response_ends_with_its_last_byte() {
    ends_at_last_byte(SIZED, "http", Phase::Done);
}

#[test]
fn a_chunked_response_ends_with_its_terminator() {
    ends_at_last_byte(CHUNKED, "http", Phase::Done);
}

#[test]
fn a_response_with_no_framing_ends_on_quiet() {
    let mut w = FakeWire::at(0);
    let mut f = reading("http");
    w.deliver(11, b"HTTP/1.1 200 OK\r\n\r\nno length");
    run(&mut w, &mut f, 30);
    w.advance(2_000);
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::ReadBody, "two seconds of quiet is not yet the end");
    w.advance(1);
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::Done);
}
