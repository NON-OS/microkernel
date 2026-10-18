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

//! DDC/CI at the edges G-42 bounds, over GMBUS: a string of exactly
//! CAPS_MAX kept and one byte more refused; a reply the sink holds, by the
//! bus or by the clock, given up within one try's deadline.

use super::caps_plan::{answer, text, Plan};
use super::ci_gen::vcp_reply;
use super::sink::{Fault, Sink};
use crate::ddc::error::DdcError;
use crate::ddc::pace::Pacer;
use crate::ddc::transact::read;
use crate::engine::gmbus_xfer::ddc::GmbusDdc;
use crate::engine::gmbus_xfer::deadline::XFER_US;
use crate::model::Ticks;
use crate::tests::ddc::caps_rig::known;
use crate::tests::wire_sweep::rng::Rng;

fn caps(len: usize) -> bool {
    let (t, p) = (Ticks::new(), Plan { text: text(len), chunk: 32, lies: Vec::new() });
    let mut s = Sink::new(&t, Vec::new());
    s.ci = Some(Box::new(move |sent: &[u8]| answer(&p, u16::from_be_bytes([sent[3], sent[4]]), 0)));
    let mut pace = Pacer::new();
    while !pace.caps.settled() {
        t.now.set(t.now.get() + 60_000);
        pace.fetch_caps(&mut GmbusDdc { mmio: &mut s, clock: &t, ddi: 1 }, t.now.get());
    }
    pace.caps.caps().is_ok()
}

#[test]
fn a_string_of_caps_max_is_kept_and_one_byte_more_is_refused() {
    assert_eq!(text(1024).len(), 1024);
    assert!(caps(1024));
    assert!(!caps(1025));
}

#[test]
fn a_reply_held_by_the_bus_or_the_clock_is_given_up_within_one_deadline() {
    for f in [Fault::Stall(1), Fault::NoPhase, Fault::Slow(40_000)] {
        let t = Ticks::new();
        let mut s = Sink::new(&t, Vec::new());
        let reply = vcp_reply(&mut Rng(1), 0x10);
        s.ci = Some(Box::new(move |_| reply.clone()));
        s.faults = vec![Fault::None, f];
        let got = read(&mut GmbusDdc { mmio: &mut s, clock: &t, ddi: 1 }, &t, &mut known(), 0x10);
        assert_eq!(got, Err(DdcError::Bus), "{f:?}");
        let rx = &s.log[1];
        assert!(rx.t1 - rx.t0 <= XFER_US + 41_000, "{f:?} held {}", rx.t1 - rx.t0);
    }
}
