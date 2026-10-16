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

//! A join that cannot have its own address is not made: not when the radio
//! refuses it, and not without randomness to make it from.

use super::hidden_tests::hidden;
use crate::soft::air::STA;
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};
use crate::soft::rig::rig;
use crate::soft::wire::{code, connect};
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::control::result::{CODE_NO_ADDRESS, CODE_NO_ENTROPY};
use nonos_wifi_core::softmac::engine::WIFI_MAGIC;
use nonos_wifi_core::softmac::join::FLAG_HIDDEN;
use nonos_wifi_core::softmac::{Env, SoftStation};

#[test]
fn a_radio_that_cannot_take_the_address_joins_nothing() {
    let mut st = rig(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    st.radio_mut().unwrap().addr.1 = true;
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), CODE_NO_ADDRESS);
    assert!(st.radio().unwrap().sent.iter().all(|f| f[0] != 0xb0), "no authentication sent");
    let mut st = hidden();
    st.radio_mut().unwrap().addr.1 = true;
    assert_eq!(code(&connect(&mut st, SSID, PASS, FLAG_HIDDEN)), CODE_NO_ADDRESS);
    assert!(st.radio().unwrap().sent.is_empty(), "not even a probe");
}

/// An environment whose clock runs and which has no randomness at all.
struct NoRandom(core::cell::Cell<u64>);

impl Env for NoRandom {
    fn now_ms(&self) -> u64 {
        self.0.set(self.0.get() + 1);
        self.0.get()
    }
    fn random(&mut self, _out: &mut [u8]) -> bool {
        false
    }
}

#[test]
fn without_randomness_no_join_is_made() {
    let air = crate::soft::air_new::air(SimAp::new(SSID, PASS, 6, &RSNE_PSK));
    let mut st = SoftStation::new(Some(air), NoRandom(Default::default()), 0);
    let mut req = WIFI_MAGIC.to_le_bytes().to_vec();
    req.extend_from_slice(&1u16.to_le_bytes());
    req.extend_from_slice(&7u32.to_le_bytes());
    req.extend_from_slice(
        &[&[SSID.len() as u8][..], SSID, &[PASS.len() as u8], PASS, &[0]].concat(),
    );
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = st.handle(&req, &mut out);
    assert_eq!(code(&out[10..n]), CODE_NO_ENTROPY);
    assert!(st.radio().unwrap().sent.is_empty(), "nothing sent");
    assert_eq!(st.radio().unwrap().addr.0, STA, "and no network's address taken");
}
