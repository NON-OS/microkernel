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

//! The Global Lock the capsule held, released (ACPI 6.5, 5.2.10.1,
//! recalled: bit 0 pending, bit 1 owned): the word with both cleared and
//! the rest kept, firmware told by GBL_RLS (PM1 control bit 2) only when it
//! set pending, and that write never setting SLP_EN (bit 13).

use super::bus::Bus;
use super::fadt_decode::FadtInfo;
use super::gas::Gas;
use super::leave_events::{global_release, release_word};

#[test]
fn the_word_is_released_and_pending_reported() {
    let high = [0u32, 4, 0x100, 0xFFFF_FFF0];
    for word in (0..4u32).flat_map(|low| high.map(|h| h | low)) {
        let (new, pending) = release_word(word);
        assert_eq!(new, word & !3, "{word:#x}");
        assert_eq!(pending, word & 1 == 1, "{word:#x}");
    }
}

#[test]
fn gbl_rls_keeps_control_but_never_sleeps() {
    let io = |port| Gas { space: 1, bit_width: 16, bit_offset: 0, access_size: 0, address: port };
    for (b, before) in [(false, 0x0001u16), (true, 0x2001), (true, 0x3C01), (false, 0xFFFF)] {
        let f = FadtInfo {
            pm1a_cnt: io(0x404),
            pm1b_cnt: if b { io(0x484) } else { Gas::empty() },
            ..FadtInfo::default()
        };
        let mut bus = Bus::default();
        for port in [0x404u64, 0x484] {
            bus.bytes.insert((false, port), before as u8);
            bus.bytes.insert((false, port + 1), (before >> 8) as u8);
        }
        global_release(&mut bus, &f);
        let after = |p: u64| u16::from(bus.byte(false, p)) | u16::from(bus.byte(false, p + 1)) << 8;
        let want = (before & !(1 << 13)) | 1 << 2;
        assert_eq!(after(0x404), want, "{before:#x}");
        assert_eq!(after(0x484), if b { want } else { before }, "{before:#x}");
    }
}
