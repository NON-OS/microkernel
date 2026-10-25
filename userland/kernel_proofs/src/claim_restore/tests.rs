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

use super::restore_rule::{holds, put, slots, wants, Kind, Want};

#[test]
fn a_bar_reset_to_zero_is_seen_lost_and_put_back_with_its_type_bits() {
    let (low, high) = wants(Kind::Mem64, 0x0000_0060_F000_0000);
    let (low, high) = (low.expect("low"), high.expect("high"));
    // After a reset: zero address, the device's own 64-bit prefetchable type.
    let reset = 0x0000_000C;
    assert!(!holds(reset, low));
    assert_eq!(put(reset, low), 0xF000_000C);
    assert!(holds(put(reset, low), low));
    assert!(!holds(0, high) && put(0, high) == 0x60 && holds(0x60, high));
}

#[test]
fn every_kind_compares_only_its_address_bits() {
    let w = wants(Kind::Mem32, 0xFEB0_0000).0.expect("low");
    assert!(holds(0xFEB0_0008, w), "a prefetchable bit is the device's");
    assert!(!holds(0xFEB1_0000, w));
    let io = wants(Kind::Io, 0xE000).0.expect("low");
    assert!(holds(0xE001, io) && !holds(0x0001, io));
    assert_eq!(put(0x0001, io), 0xE001);
    assert_eq!(wants(Kind::Absent, 0xFFFF_FFFF), (None, None), "never one it found absent");
    assert_eq!(wants(Kind::Mem32, 0x1_0000_0000).1, None, "a 32-bit BAR has no high half");
}

#[test]
fn put_never_writes_outside_the_address_bits_it_was_given() {
    for read in [0u32, 0xF, 0xFFFF_FFFF, 0x1234_5678] {
        for want in [Want { value: 0xABCD_EF00, mask: !0xF }, Want { value: 0x5000, mask: !0x3 }] {
            let written = put(read, want);
            assert_eq!(written & !want.mask, read & !want.mask, "{read:#x}");
            assert!(holds(written, want));
        }
    }
}

#[test]
fn a_64_bit_bar_fills_the_next_slot_with_its_high_half_and_no_other() {
    use Kind::{Absent, Mem32, Mem64};
    let high = |v| Some(Want { value: v, mask: u32::MAX });
    let mut bars = [(Absent, 0); 6];
    (bars[0], bars[2]) = ((Mem32, 0xFEB0_0000), (Mem64, 0x60_F000_0000));
    let s = slots(bars);
    assert_eq!((s[0], s[1], s[3], s[5]), (wants(Mem32, 0xFEB0_0000).0, None, high(0x60), None));
    bars = [(Absent, 0); 6];
    bars[4] = (Mem64, 1 << 32);
    assert_eq!(slots(bars)[5], high(1), "slot 4's high half in slot 5");
    // A decoded BAR keeps its own slot; past the last slot nothing is written.
    bars = [(Absent, 0); 6];
    (bars[0], bars[1], bars[5]) = ((Mem64, 2 << 32), (Mem32, 0xFE00_0000), (Mem64, 3 << 32));
    let both = slots(bars);
    assert_eq!((both[1], both[5]), (wants(Mem32, 0xFE00_0000).0, wants(Mem64, 3 << 32).0));
}
