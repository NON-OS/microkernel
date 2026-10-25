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

//! The one other way after a reset whose marker survived, and which functions
//! may reach memory past Bus Master off (L78). The fallback is never the way
//! just tried, so a function that ignores a way is never given it twice, and
//! it is only ever the D3hot trip where that is offered; after the trip, or
//! where it is not offered, there is none and the wipe ends Unreset. Binding
//! is VMMDev, or an Intel display function, over every vendor and class.

use super::method_rule::{binds_past_bus_master, fallback, method, Method, Offers};

const ALL: [Method; 3] = [Method::Flr, Method::AfFlr, Method::PmCycle];

#[test]
fn the_fallback_is_never_the_way_tried_and_only_ever_the_d3hot_trip() {
    let mut seen = [0u32; 3];
    for bits in 0..8u8 {
        let o = Offers { flr: bits & 1 != 0, af_flr: bits & 2 != 0, soft_reset_d3: bits & 4 != 0 };
        for tried in ALL {
            let next = fallback(tried, o);
            assert_ne!(next, Some(tried), "{tried:?} given twice, {o:?}");
            let want = (tried != Method::PmCycle && o.soft_reset_d3).then_some(Method::PmCycle);
            assert_eq!(next, want, "{tried:?} {o:?}");
            seen[usize::from(next.is_some())] += 1;
        }
        // From the first way, at most one more, then none: the wipe ends.
        let second = method(o).and_then(|m| fallback(m, o));
        assert_eq!(second.and_then(|m| fallback(m, o)), None, "{o:?}");
        seen[2] += u32::from(second.is_some());
    }
    // A second way exists only after an FLR of either kind, with the trip offered.
    assert_eq!(seen, [16, 8, 3], "{seen:?}");
}

#[test]
fn only_vmmdev_or_an_intel_display_function_binds_past_bus_master() {
    let mut binds = 0u32;
    for vendor in 0..=u16::MAX {
        for class in 0..=u8::MAX {
            // Intel's vendor ID and the display class (recalled, unverified).
            let intel_display = vendor == 0x8086 && class == 0x03;
            assert_eq!(binds_past_bus_master(false, vendor, class), intel_display);
            assert!(binds_past_bus_master(true, vendor, class), "VMMDev, whatever it says");
            binds += u32::from(intel_display);
        }
    }
    assert_eq!(binds, 1, "one vendor and class pair binds");
    assert!(!binds_past_bus_master(false, 0x8086, 0x02), "an Intel network function");
    assert!(!binds_past_bus_master(false, 0x1234, 0x03), "another vendor's display");
}
