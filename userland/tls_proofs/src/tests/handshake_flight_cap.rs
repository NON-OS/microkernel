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

//! The flight cap, 128 KiB, holds over every record of a flight, not each
//! one: an honest ServerHello followed by ChangeCipherSpec records, each
//! six bytes and each skipped, is refused once their sum passes the cap,
//! on the stream path and the one-shot path alike.

use std::vec::Vec;

use crate::session::{exchange, Io, SessionError};
use crate::stream::connect_unauthenticated;

use super::fake_server::Wire;

const CAP: usize = 128 * 1024;

/// The ServerHello record of the honest flight, then CCS records forever.
struct Flood {
    wire: Wire,
    hello: Vec<u8>,
    sent: usize,
}

impl Io for Flood {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.wire.write_all(data)?;
        let mut all = [0u8; 4096];
        let n = self.wire.read(&mut all)?;
        let len = usize::from(u16::from_be_bytes([all[3], all[4]]));
        self.hello = all[..(5 + len).min(n)].to_vec();
        Ok(())
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        let ccs = [20u8, 3, 3, 0, 1, 1];
        let stream = self.hello.iter().copied().chain(ccs.iter().copied().cycle());
        let n = into.len().min(4096);
        for (slot, b) in into[..n].iter_mut().zip(stream.skip(self.sent)) {
            *slot = b;
        }
        self.sent += n;
        Ok(n)
    }
}

fn flood() -> Flood {
    Flood { wire: Wire::new(), hello: Vec::new(), sent: 0 }
}

#[test]
fn a_flight_of_many_small_records_is_refused_past_the_cap() {
    let mut f = flood();
    let got = connect_unauthenticated(&mut f, b"relay");
    assert_eq!(got.err(), Some(SessionError::TooLarge));
    assert!(f.sent <= CAP + 4096, "read no more than the cap and one chunk: {}", f.sent);
    assert!(f.sent > CAP - 4096, "many records, all counted: {}", f.sent);
    let mut f = flood();
    let got = exchange(&mut f, "relay", b"GET /", 0, 1 << 16);
    assert_eq!(got.err(), Some(SessionError::TooLarge), "exchange's cap is the same");
    assert!(f.sent <= CAP + 4096);
}
