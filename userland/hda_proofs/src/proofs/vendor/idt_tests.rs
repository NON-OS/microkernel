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
//! IDT's port power map (controller/codec/idt): which families have one and
//! over which ports, a port powered down only when its pin says nothing is
//! connected, and the map written to the function group.

use crate::controller::codec::idt::{ports, power_map, SET_POWER_MAP};
use crate::proofs::fixtures::{alc236_hp, codec, pin, Desc, NC, PIN_OUT_WCAPS};
use crate::proofs::program_sim::{at, programmed, short};

#[test]
fn each_family_has_its_ports_and_no_other_codec_has_a_map() {
    for (vendor, n) in [(0x111d_7674u32, 8), (0x111d_76c5, 8), (0x111d_7605, 7), (0x111d_7668, 7)] {
        assert_eq!(ports(vendor).len(), n, "{vendor:#x}");
    }
    for (vendor, n) in [(0x111d_76b7u32, 3), (0x111d_7603, 3), (0x111d_7695, 4)] {
        assert_eq!(ports(vendor).len(), n, "{vendor:#x}");
    }
    for vendor in [0x111d_7600u32, 0x8384_7680, 0x10ec_0236, 0x14f1_50f2] {
        assert!(ports(vendor).is_empty(), "{vendor:#x}");
    }
}

#[test]
fn only_a_port_whose_pin_says_nothing_is_there_is_powered_down() {
    let out = |nid, cfg| pin(nid, PIN_OUT_WCAPS, 0x0001_0014, cfg, &[0x02]);
    let mut d = Desc { vendor: 0x111d_76b0, ..alc236_hp() };
    d.widgets.extend([out(0x0a, 0x0221_1020), out(0x0d, NC), out(0x0f, 0x9017_0110)]);
    assert_eq!(power_map(&codec(&d, 0)), Some(0b010), "0x0d, the second of 0x0a, 0x0d, 0x0f");
    d.vendor = 0x10ec_0236;
    assert_eq!(power_map(&codec(&d, 0)), None, "not IDT's");
}

#[test]
fn the_map_is_written_to_the_function_group() {
    let (_s, sent) = programmed(&Desc { vendor: 0x111d_76d1, ..alc236_hp() }, false);
    at(&sent, short(1, SET_POWER_MAP, 0));
    let (_s, sent) = programmed(&alc236_hp(), false);
    assert!(!sent.contains(&short(1, SET_POWER_MAP, 0)), "a Realtek codec has no map");
}
