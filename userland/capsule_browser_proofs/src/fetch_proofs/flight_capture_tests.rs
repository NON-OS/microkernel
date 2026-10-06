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

//! A handshake flight is judged when its Finished is in, and not before.

use super::fetch_fixtures::{awaiting_flight, CAPTURED_FLIGHT};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::run::run;
use crate::browser::fetch::types::Phase;
use crate::browser::tls13;

/*
 * The capture's encrypted flight is one record, which the old rule, three
 * encrypted records or fifteen quiet ticks, never recognised: the deadline
 * ran out first and the page said "timed out". The client's private key for
 * the capture is gone, so the record opens under no key a proof holds; what
 * is proven is when the flight is judged. Split at every byte, the fetch
 * waits until the last byte and is judged in the call that brings it.
 */
#[test]
fn the_captured_one_record_flight_is_judged_at_its_last_byte() {
    let last = CAPTURED_FLIGHT.len();
    for cut in 1..last {
        let mut w = FakeWire::at(0);
        let cf = tls13::client_flight(b"nonos.software").expect("client hello");
        let mut f = awaiting_flight("https://nonos.software/", 11, 0, cf);
        w.deliver(11, &CAPTURED_FLIGHT[..cut]);
        run(&mut w, &mut f, 30);
        assert_eq!(f.phase, Phase::TlsFlight, "still waiting after {cut} of {last} bytes");
        w.deliver(11, &CAPTURED_FLIGHT[cut..]);
        run(&mut w, &mut f, 30);
        assert_eq!(f.phase, Phase::Error, "judged with the last byte, cut at {cut}");
        assert_eq!(f.error, Some("tls handshake failed"), "a key it was not sealed for");
        assert_eq!(f.received, last);
    }
}
