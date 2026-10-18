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

//! The config-space snapshot the transport choice reads, through the broker.

use super::broker::FakeBroker;
use super::space::{qemu_modern, qemu_modern_bars, MODERN_NET};
use crate::caps::parse;
use crate::pci::ConfigSpace;

#[test]
fn the_snapshot_read_through_the_broker_is_the_config_space() {
    let space = qemu_modern(MODERN_NET);
    let mut broker = FakeBroker::new(&space, &qemu_modern_bars());
    let cfg = ConfigSpace::read(&mut broker).expect("every read answered");
    assert_eq!(broker.reads, 64, "one dword read per four bytes");
    assert_eq!(parse(&cfg, &qemu_modern_bars()), parse(&space.cfg(), &qemu_modern_bars()));
    // One refused read refuses the snapshot: a partial one would parse as
    // zeros and could hide the capability list.
    broker.refuse_read = Some(0x70);
    assert!(ConfigSpace::read(&mut broker).is_none());
}
