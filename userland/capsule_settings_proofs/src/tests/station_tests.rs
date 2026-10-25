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
//! The Network page reads the card's address source as the driver writes it
//! (nonos_usbnet's own encoder), and each source has its words, or none.

#[path = "../../../nonos_usbnet/src/nnet/counters.rs"]
mod counters;
#[allow(dead_code)]
#[path = "../../../nonos_usbnet/src/nnet/encode.rs"]
mod encode;
#[path = "../../../capsule_settings/src/wifi/station.rs"]
pub(super) mod station;
#[path = "../../../capsule_settings/src/wifi/station_words.rs"]
pub(super) mod station_words;
#[allow(dead_code)]
#[path = "../../../nonos_usbnet/src/nnet/stats.rs"]
mod stats;
#[allow(dead_code)]
#[path = "../../../nonos_usbnet/src/nnet/wire.rs"]
mod wire;

use station::{station_of, stats_request, REPLY_LEN};

fn answer(id: u32, source: u32, status: i32) -> Vec<u8> {
    let req = wire::decode(&stats_request(id)).expect("the driver reads our request");
    assert_eq!(req.op, wire::OP_STATS);
    let st = stats::Stats { station: source, tx_frames: 9, ..Default::default() };
    let mut tx = vec![0u8; REPLY_LEN];
    let n = st.encode(&mut tx[wire::DATA_AT..]);
    let len = encode::reply(&mut tx, &req, status, n);
    tx.truncate(len);
    tx
}

#[test]
fn the_page_reads_the_word_the_driver_writes() {
    for source in [0, 1, stats::STATION_HOST_UNANSWERED, stats::STATION_HOST_REFUSED] {
        assert_eq!(station_of(&answer(7, source, 0), 7), Some(source));
    }
}

#[test]
fn anything_but_our_ok_answer_is_none() {
    let ok = answer(7, 3, 0);
    assert_eq!(station_of(&ok, 8), None, "another request's answer");
    assert_eq!(station_of(&answer(7, 3, -5), 7), None, "an error status");
    assert_eq!(station_of(&ok[..ok.len() - 1], 7), None, "cut short");
    let mut other = ok.clone();
    other[6] = 2;
    assert_eq!(station_of(&other, 7), None, "another op");
    let mut foreign = ok.clone();
    foreign[0] ^= 1;
    assert_eq!(station_of(&foreign, 7), None, "not NNET");
}
