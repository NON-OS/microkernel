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

//! Get VCP over GMBUS against seeded replies and bus faults. Each read is
//! held to the test's own reading: a request refused is Absent, a reply
//! not taken off the bus is Bus, else the reply's own rule decides; the
//! request is the bytes the layout gives, no reply read past its eleven
//! bytes' words, and no try past its deadline.

use super::ci_bus::taken;
use super::ci_gen::vcp_reply;
use super::ci_own::{request, vcp};
use super::draw::fault;
use super::sink::{Fault, Sink};
use crate::ddc::error::DdcError;
use crate::ddc::transact::read;
use crate::engine::gmbus_xfer::ddc::GmbusDdc;
use crate::engine::gmbus_xfer::deadline::XFER_US;
use crate::model::Ticks;
use crate::tests::ddc::caps_rig::known;
use crate::tests::wire_sweep::rng::Rng;

#[test]
fn a_vcp_reply_over_gmbus_is_believed_only_whole() {
    let (mut r, mut seen) = (Rng(0x6900_0C1A), std::collections::HashMap::new());
    for _ in 0..4_000 {
        let (t, code) = (Ticks::new(), [0x10, 0x12, 0x60][r.below(3) as usize]);
        let reply = vcp_reply(&mut r, code);
        let mut s = Sink::new(&t, Vec::new());
        s.ci = Some(Box::new(move |_| reply.clone()));
        s.ci_nak = r.below(12) == 0;
        s.faults = vec![Fault::None, if r.below(4) == 0 { fault(&mut r) } else { Fault::None }];
        let got = read(&mut GmbusDdc { mmio: &mut s, clock: &t, ddi: 1 }, &t, &mut known(), code);
        let (asked, sent) = (request(&[0x01, code]), &s.log[0].sent);
        assert!(asked.starts_with(sent) && (s.log[0].nak || *sent == asked), "the request");
        let want = match (s.log[0].nak, s.log.get(1)) {
            (true, None) => Err(DdcError::Absent),
            (false, Some(x)) if !taken(x, 11) => Err(DdcError::Bus),
            (false, Some(x)) => vcp(code, &x.got[..11]),
            _ => panic!("a reply read after a refused request, or none after a taken one"),
        };
        assert_eq!(got, want);
        for x in &s.log {
            assert!(x.got.len() <= 12 && x.t1 - x.t0 <= XFER_US + 121_000);
        }
        *seen.entry(format!("{:?}", got.map(|_| ()))).or_insert(0) += 1;
    }
    assert_eq!(seen.len(), 7, "every outcome {seen:?}");
    assert!(seen.values().all(|&c| c >= 40), "{seen:?}");
}
