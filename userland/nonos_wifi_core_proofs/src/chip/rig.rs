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
//! A station over the lying chip, with one AP that sends data once keyed.

use super::liar::Liar;
use crate::soft::air_new::air;
use crate::soft::ap::SimAp;
use crate::soft::env::SimEnv;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};
use crate::soft::wire::code;
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::engine::WIFI_MAGIC;
use nonos_wifi_core::softmac::SoftStation;

pub type Lied = SoftStation<Liar, SimEnv>;

pub fn station(seed: u64, flood: bool) -> Lied {
    let mut ap = SimAp::new(SSID, PASS, 6, &RSNE_PSK);
    ap.chatter = true;
    let air = air(ap);
    let clock = air.clock.clone();
    SoftStation::new(Some(Liar { air, seed, flood, calls: 0 }), SimEnv { clock, draws: 0 }, 0)
}

pub fn connect(st: &mut Lied) -> i32 {
    let mut req = WIFI_MAGIC.to_le_bytes().to_vec();
    req.extend_from_slice(&1u16.to_le_bytes());
    req.extend_from_slice(&7u32.to_le_bytes());
    req.extend_from_slice(
        &[&[SSID.len() as u8][..], SSID, &[PASS.len() as u8], PASS, &[0]].concat(),
    );
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = st.handle(&req, &mut out);
    code(&out[10..n])
}
