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

//! A good part, brought up: the rings the manuals lay out, every receive
//! buffer the part's, the perfect filter loaded with the drawn address and
//! broadcast alone, and promiscuous mode left off.

use super::util::{drawn, up};
use crate::bench::{Bench, CONTROL_DEV, RX_DEV};
use crate::init::finish;
use crate::model::{take, Event, RS_WAITING};

#[test]
fn the_part_gets_its_rings_and_the_drawn_address() {
    let mut bench = Bench::new();
    let driver = up(&mut bench);
    assert_eq!(driver.mac, drawn());
    let part = take();
    assert_eq!((part.csr[3], part.csr[4]), (CONTROL_DEV, CONTROL_DEV + 0x200));
    assert_eq!((part.csr[0], part.csr[6], part.csr[7]), (0x0800, 0x2002, 0), "DSL 0, no PR");
    assert_eq!(part.rs, RS_WAITING);
    for i in 0..32u32 {
        let d = CONTROL_DEV + i * 16;
        assert_eq!(part.mem.u32(d), 0x8000_0000, "slot {i} handed over");
        let rer = if i == 31 { 1 << 25 } else { 0 };
        assert_eq!(part.mem.u32(d + 4), 1536 | rer, "slot {i}'s size and end of ring");
        assert_eq!(part.mem.u32(d + 8), RX_DEV + i * 1536);
    }
    assert_eq!(part.filter[0], drawn());
    assert_eq!(part.filter[1], [0xFF; 6]);
    assert!(part.filter[2..].iter().all(|a| *a == drawn()), "no third address");
    assert!(nonos_libc::given_back().is_empty());
    assert!(nonos_libc::logged()[0].starts_with("driver.tulip: up, 1011:0019 csr5="));
}

// The filter is loaded before a single frame can be received.
#[test]
fn reset_then_rings_then_setup_then_receive() {
    let mut bench = Bench::new();
    up(&mut bench);
    let log = take().log;
    let at = |want: &dyn Fn(&Event) -> bool| log.iter().position(want).expect("never happened");
    let reset = at(&|e| matches!(e, Event::Reset { .. }));
    let rings = at(&|e| matches!(e, Event::Csr(4, _)));
    let setup = at(&|e| *e == Event::Setup);
    let receive = at(&|e| matches!(e, Event::Csr(6, v) if v & 2 != 0));
    assert!(reset < rings && rings < setup && setup < receive, "{log:?}");
}

// A boot ROM can leave the part receiving into memory the driver never
// granted; it is stopped before anything else is written.
#[test]
fn a_part_left_running_is_stopped_first() {
    let mut bench = Bench::new();
    finish(bench.with(|p| p.csr[6] = 0x2002)).expect("bring-up");
    assert_eq!(take().log[0], Event::Csr(6, 0));
}
