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


//! Throughput: one pump call reads every cell waiting on the link, not one.
//!
//! The pump used to take a single cell per call and the idle path called it
//! once per turn, so a stream moved at four or five cells a second. Here a
//! service answers with 200 KB at once and a single pump call must deliver
//! it, with the SENDMEs that keep the relay sending paid along the way.

use super::world::{net, Faults};
use crate::manager::{pump_tick, send_data};

#[test]
fn one_pump_call_drains_a_large_answer() {
    let mut n = net(Faults::default(), |_| {});
    n.world.service_reply = std::vec![0x5a; 200 * 1024];
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");

    assert!(send_data(&mut n.state, id, b"GET / HTTP/1.0\r\n\r\n").is_ok());
    // Let the network answer, without pumping.
    let link = n.state.link.as_mut().unwrap();
    n.world.serve(link);
    let waiting = n.state.link.as_ref().unwrap().inbound.len();
    assert!(waiting > 400, "the answer is hundreds of cells: {waiting}");

    pump_tick(&mut n.state, n.now);
    let left = n.state.link.as_ref().unwrap().inbound.len();
    let got = n.state.streams.iter().find(|s| s.id == id).map(|s| s.inbound.len()).unwrap_or(0);
    assert_eq!(left, 0, "one call read every waiting cell");
    assert_eq!(got, 200 * 1024, "and delivered all of it");
}
