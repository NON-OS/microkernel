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

//! A good part, brought up: the init block is the one the data sheet lays
//! out, every receive buffer is the part's, and the APROM is never read.

use super::util::{drawn, position, up};
use crate::bench::{Bench, CONTROL_DEV, RX_DEV};
use crate::init::finish;
use crate::model::{take, Event};

#[test]
fn the_part_gets_its_rings_and_the_drawn_address() {
    let mut bench = Bench::new();
    let driver = up(&mut bench);
    assert_eq!((driver.part, driver.mac), (0x2621, drawn()));
    let part = take();
    assert_eq!(part.csr[0] & 0x0036, 0x0032, "running, not stopped");
    assert_eq!((part.bcr[20], part.csr[3], part.csr[4]), (0x0302, 0x5F00, 0x0915));
    let r = part.rings;
    assert_eq!((r.mode, r.rx_len, r.tx_len, r.ladrf), (0, 32, 32, [0; 8]));
    assert_eq!((r.padr, r.rdra, r.tdra), (drawn(), CONTROL_DEV + 0x100, CONTROL_DEV + 0x300));
    for i in 0..32u32 {
        let d = r.rdra + i * 16;
        assert_eq!(part.mem.u16(d + 6), 0x8000, "slot {i} handed over");
        assert_eq!(part.mem.u16(d + 4), 0xFA00, "-1536 with its ones");
        assert_eq!(part.mem.u32(d), RX_DEV + i * 1536);
        assert_eq!(part.mem.u16(r.tdra + i * 16 + 6), 0, "transmit slot {i} empty");
    }
    assert!(!part.log.contains(&Event::ApromRead), "the factory address was read");
    assert!(nonos_libc::given_back().is_empty());
    assert!(nonos_libc::logged()[0].starts_with("driver.pcnet: up, part=2621 csr0="));
}

#[test]
fn reset_then_style_then_init_then_start() {
    let mut bench = Bench::new();
    up(&mut bench);
    let log = take().log;
    let reset = position(&log, |e| matches!(e, Event::Reset { .. }));
    let style = position(&log, |e| *e == Event::Bcr(20, 2));
    let init = position(&log, |e| *e == Event::Init);
    let start = position(&log, |e| matches!(e, Event::Csr0(v) if v & 2 != 0));
    assert!(reset < style && style < init && init < start, "{log:?}");
}

// A boot ROM can leave the part receiving into memory the driver never
// granted; it is stopped before anything else is written.
#[test]
fn a_part_left_running_is_stopped_first() {
    let mut bench = Bench::new();
    finish(bench.with(|p| p.csr[0] = 0x0033)).expect("bring-up");
    assert_eq!(take().log[0], Event::Csr0(0x0004));
}
