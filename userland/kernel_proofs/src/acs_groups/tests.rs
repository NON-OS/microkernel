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

use super::acs_rule::{closed, closing, find_ext, ACS_ID, DIRECT, NEEDED};
use std::cell::Cell;

const SV: u16 = 1 << 0;
const TB: u16 = 1 << 1;
const RR: u16 = 1 << 2;
const CR: u16 = 1 << 3;
const UF: u16 = 1 << 4;

#[test]
fn a_port_is_closed_only_with_all_four_controls_and_no_direct_path() {
    assert_eq!(NEEDED, SV | RR | CR | UF);
    for missing in [SV, RR, CR, UF] {
        assert_eq!(closing(NEEDED & !missing, 0), None, "{missing:#x} not offered");
        assert!(!closed(NEEDED & !missing), "{missing:#x} off");
    }
    assert_eq!(closing(0xFFFF, 0), Some(NEEDED), "every control offered");
    assert_eq!(closing(NEEDED, TB | DIRECT), Some(NEEDED | TB), "direct off, the rest kept");
    assert!(closed(NEEDED) && closed(NEEDED | TB) && !closed(NEEDED | DIRECT));
    assert_eq!(closing(NEEDED, 0).map(closed), Some(true));
}

// A config space of (offset, header) pairs, counting reads.
fn space<'a>(list: &[(u16, u32)], reads: &'a Cell<u32>) -> impl Fn(u16) -> u32 + 'a {
    let list = list.to_vec();
    move |at| {
        assert!((0x100..0x1000).contains(&at) && at & 3 == 0, "read at {at:#x}");
        reads.set(reads.get() + 1);
        list.iter().find(|&&(o, _)| o == at).map_or(0, |&(_, h)| h)
    }
}

fn header(id: u16, next: u16) -> u32 {
    (next as u32) << 20 | 1 << 16 | id as u32
}

#[test]
fn the_capability_walk_ends_on_every_chain_firmware_could_write() {
    let reads = Cell::new(0);
    assert_eq!(ACS_ID, 0x000D);
    let chain = [(0x100, header(0x0001, 0x148)), (0x148, header(ACS_ID, 0))];
    assert_eq!(find_ext(space(&chain, &reads), ACS_ID), Some(0x148));
    assert_eq!(find_ext(space(&chain, &reads), 0x0010), None, "the end of the list");
    let looped = [(0x100, header(0x0001, 0x200)), (0x200, header(0x0002, 0x100))];
    reads.set(0);
    assert_eq!(find_ext(space(&looped, &reads), 0x000D), None);
    assert!(reads.get() <= 960, "a loop is cut at the dwords there are");
    let back = [(0x100, header(0x0001, 0x0FC))];
    assert_eq!(find_ext(space(&back, &reads), 0x000D), None, "never below 100h");
    let ones = [(0x100, !0)];
    assert_eq!(find_ext(space(&ones, &reads), 0xFFFF), None, "absent, not a match");
    let crooked = [(0x100, header(0x0001, 0x14A)), (0x148, header(0x000D, 0))];
    assert_eq!(find_ext(space(&crooked, &reads), 0x000D), Some(0x148), "dword aligned");
}
