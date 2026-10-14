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

//! What a client may reach once setup has bound a controller. Multi-SKU
//! firmware declares every pad a chassis may carry and only the fitted one
//! answers, and maybe not yet: so until a device returns its HID
//! descriptor, every pad declared on this controller, or on none, stays in
//! reach, and once one has, it alone does.

use nonos_i2cmodel::Config;

use super::declared::{here, nowhere, on, reached, this};
use super::fixture::{bench, core, PAD};
use crate::setup::settle::settle;
use crate::transaction::Presence;

#[test]
fn a_descriptor_puts_the_pad_alone_in_reach() {
    let b = bench(Config::LPSS);
    let (pad, other) = (here(PAD), here(0x2C));
    let s = settle(&b.driver, &this(), &pad, Presence::HidDescriptor(0x20), &[other, pad]);
    assert_eq!(reached(&s.reach), [PAD]);
    assert_eq!((s.addr, s.desc_reg, s.by_probe), (PAD, 0x20, true));
}

#[test]
fn before_a_descriptor_each_pad_declared_here_or_nowhere_stays_in_reach() {
    let b = bench(Config::LPSS);
    let (first, second, elsewhere, unnamed) = (here(0x2C), here(PAD), on(3, 0x10), nowhere(0x20));
    let targets = [first, elsewhere, second, unnamed];
    for found in [Presence::Acked, Presence::Absent] {
        let s = settle(&b.driver, &this(), &first, found, &targets);
        assert_eq!(reached(&s.reach), [0x15, 0x20, 0x2C], "{found:?}");
        assert_eq!((s.addr, s.desc_reg), (0x2C, 0x0001), "{found:?}");
        assert_eq!(s.by_probe, found == Presence::Acked);
    }
    assert!(core(|c| c.bus().trace().is_empty()), "settling an addressed pad probed the bus");
}
