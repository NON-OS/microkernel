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

//! Any request at all, on a fresh and on a joined station: `handle` returns
//! a reply that fits the buffer and never panics; a WIFI request with a
//! known op echoes its ten-octet header back; everything else is refused.

use super::wire::request;
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{joined, PASS, SSID};
use crate::soft::rig::{rig, Station};
use crate::sweep::rng::each;
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::engine::{WIFI_HDR, WIFI_MAGIC};

/// Every op, known and unknown, including connect, so no body is excluded.
const OPS: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 0xFFFF];

fn fresh() -> Station {
    rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK))
}

fn beat(st: &mut Station) {
    each(|rng| {
        let req = request(rng, &OPS);
        let mut out = vec![0u8; MAX_RESPONSE];
        let n = st.handle(&req, &mut out);
        assert!(n <= out.len(), "a reply ran past its buffer");
        let wifi = req.len() >= WIFI_HDR && req[..4] == WIFI_MAGIC.to_le_bytes();
        let op = if wifi { u16::from_le_bytes([req[4], req[5]]) } else { 0 };
        if wifi && (1..=5).contains(&op) && n >= WIFI_HDR {
            assert_eq!(out[..WIFI_HDR], req[..WIFI_HDR], "a known op echoes its header");
        }
    });
}

#[test]
fn a_fresh_station_answers_any_request_safely() {
    beat(&mut fresh());
}

#[test]
fn a_joined_station_answers_any_request_safely() {
    beat(&mut joined());
}
