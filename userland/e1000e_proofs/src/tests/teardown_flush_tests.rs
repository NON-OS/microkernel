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

//! An I219 torn down with its rings still holding descriptors. Linux's
//! e1000e_reset flushes them first: a reset that lands on a part holding
//! descriptors is a unit hang only a PCI reset clears. The teardown does the
//! same, on this driver's own rings, then stops and resets the part and
//! turns Bus Master off before the device goes back.

use nonos_libc::{entropy, logged, seen, set_ring_status, watch};

use super::order::{no_reset_while_mastering, released_quiet};
use super::probe::{probe, resets};
use crate::constants::Family;
use crate::init::finish::finish;
use crate::model::live::live;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::{pch, phy_at};

#[test]
fn an_i219_torn_down_with_rings_pending_is_flushed_before_its_reset() {
    let _turn = entropy(true);
    let bar = pch();
    let part = live(&bar, phy_at(2), Behaviour::default());
    watch(probe(&bar, &part.how));
    let mut mem = Memory::new();
    let d = finish(mem.driver(&bar, Family::PchCnp, 0x15BB)).expect("brought up");
    let (mark, said, done) = (seen().len(), logged().len(), resets(&part.how));
    set_ring_status(0x100);
    d.release();
    let flushing = "e1000e: descriptor rings left pending, flushing";
    assert_eq!(logged()[said..].first().map(String::as_str), Some(flushing));
    assert_eq!(mem.tx[0].length, 512, "the flush descriptor went in slot 0");
    released_quiet(&seen()[mark..], done + 1);
    no_reset_while_mastering(&seen(), resets(&part.how));
}
