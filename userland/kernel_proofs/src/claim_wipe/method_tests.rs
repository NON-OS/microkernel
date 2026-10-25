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

//! Which reset a function is given (L78), over Device Capabilities with and
//! without its FLR bit among noise, every Advanced Features capabilities
//! byte, and PMCSR in each power state with No_Soft_Reset set or clear, each
//! also absent: an FLR wherever one is offered, else the Advanced Features
//! FLR, else the trip to D3hot only where No_Soft_Reset is clear (the power
//! state the function sat in at enumeration never matters), else none.

use super::method_rule::{method, offers, Method, Offers};

/// Device Capabilities' FLR bit (recalled, unverified).
const FLR: u32 = 1 << 28;

#[test]
fn offers_and_the_method_follow_flr_then_af_flr_then_the_d3hot_trip() {
    let devcaps = [None, Some(0), Some(FLR), Some(!FLR), Some(u32::MAX), Some(0x0000_8FC2)];
    let afs: Vec<Option<u8>> = std::iter::once(None).chain((0..=255).map(Some)).collect();
    // PowerState, No_Soft_Reset, and PME_Status with PME_En as noise.
    let mut pmcsrs = vec![None];
    for state in 0..4u16 {
        for no_soft_reset in [0, 1 << 3] {
            pmcsrs.extend([0, 0x8100].map(|noise| Some(state | no_soft_reset | noise)));
        }
    }
    let mut seen = [0u32; 4];
    for (devcap, af, pmcsr) in
        devcaps.iter().flat_map(|d| afs.iter().flat_map(|a| pmcsrs.iter().map(|p| (*d, *a, *p))))
    {
        let o = offers(devcap, af, pmcsr);
        let want = Offers {
            flr: devcap.is_some_and(|c| c & FLR != 0),
            af_flr: af.is_some_and(|c| c & 2 != 0),
            soft_reset_d3: pmcsr.is_some_and(|p| p & 1 << 3 == 0),
        };
        assert_eq!(o, want, "{devcap:x?} {af:x?} {pmcsr:x?}");
        let order =
            [(o.flr, Method::Flr), (o.af_flr, Method::AfFlr), (o.soft_reset_d3, Method::PmCycle)];
        let first = order.iter().find(|w| w.0).map(|w| w.1);
        assert_eq!(method(o), first, "{o:?}");
        seen[first.map_or(3, |m| m as usize)] += 1;
    }
    // Flr: 2 devcaps of 6; AfFlr: 4 x 128 bytes; the trip: 4 x 129 x 8 clear PMCSRs.
    assert_eq!(seen, [2 * 257 * 17, 4 * 128 * 17, 4 * 129 * 8, 4 * 129 * 9], "{seen:?}");
}

#[test]
fn a_function_offering_nothing_gets_no_method() {
    assert_eq!(offers(None, None, None), Offers::default());
    assert_eq!(method(Offers::default()), None, "no reset to trust");
    assert_eq!(method(offers(Some(!FLR), Some(!2), Some(1 << 3))), None, "every other bit");
}
