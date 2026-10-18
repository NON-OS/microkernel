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
//! A generator stuck on one value: the first scan address is fresh, the
//! next can only repeat it. Four draws, then the join answers NO_ENTROPY
//! with no probe sent, and it does not draw again.

use crate::soft::air_new::air;
use crate::soft::ap::SimAp;
use crate::soft::frames::RSNE_PSK;
use crate::soft::join::{PASS, SSID};
use crate::soft::wire::code;
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::control::result::CODE_NO_ENTROPY;
use nonos_wifi_core::softmac::engine::WIFI_MAGIC;
use nonos_wifi_core::softmac::join::FLAG_HIDDEN;
use nonos_wifi_core::softmac::{Env, Radio, SoftStation};
use std::cell::Cell;
use std::rc::Rc;

struct Stuck {
    clock: Cell<u64>,
    calls: Rc<Cell<u32>>,
}

impl Env for Stuck {
    fn now_ms(&self) -> u64 {
        self.clock.set(self.clock.get() + 1);
        self.clock.get()
    }
    fn random(&mut self, out: &mut [u8]) -> bool {
        self.calls.set(self.calls.get() + 1);
        out.fill(0x5a);
        true
    }
}

/// A hidden connect, answered with its code.
fn connect<R: Radio, E: Env>(st: &mut SoftStation<R, E>) -> i32 {
    let mut req = WIFI_MAGIC.to_le_bytes().to_vec();
    req.extend_from_slice(&1u16.to_le_bytes());
    req.extend_from_slice(&7u32.to_le_bytes());
    let body = [&[SSID.len() as u8][..], SSID, &[PASS.len() as u8], PASS, &[FLAG_HIDDEN]];
    req.extend_from_slice(&body.concat());
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = st.handle(&req, &mut out);
    code(&out[10..n])
}

#[test]
fn a_stuck_generator_gets_one_scan_then_no_entropy_and_no_probe() {
    let mut ap = SimAp::new(SSID, PASS, 6, &RSNE_PSK);
    ap.hidden = true;
    let calls = Rc::new(Cell::new(0));
    let env = Stuck { clock: Cell::new(0), calls: calls.clone() };
    let mut st = SoftStation::new(Some(air(ap)), env, 0);
    assert_eq!(connect(&mut st), 0, "the first scan address is fresh");
    st.radio_mut().unwrap().sent.clear();
    let before = calls.get();
    assert_eq!(connect(&mut st), CODE_NO_ENTROPY);
    assert_eq!(calls.get() - before, 4, "four draws, then no more");
    assert!(st.radio().unwrap().sent.iter().all(|f| f[0] != 0x40), "no probe sent");
}
