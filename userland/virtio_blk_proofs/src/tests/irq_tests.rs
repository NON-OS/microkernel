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

//! The interrupt a bring-up binds: one MSI-X vector of the device's own
//! when the function has MSI-X, its INTx line only without, and neither
//! refused by name with the claim given back.

use nonos_libc::{events, offer_irq, Event};

use super::bench::bench;
use crate::queue::ring_bytes;
use crate::setup;

fn binds() -> Vec<Event> {
    events().into_iter().filter(|e| matches!(e, Event::IrqBind { .. })).collect()
}

#[test]
fn msix_is_asked_for_first_and_the_queue_is_pointed_at_its_entry() {
    let dev = bench(8, 2048, 1);
    dev.borrow_mut().msix = true;
    setup::run().expect("the device comes up");
    assert_eq!(binds(), [Event::IrqBind { msix: true }], "INTx was asked for first");
    assert_eq!(dev.borrow().vectors, [0xFFFF, 0], "queue on entry 0, config changes on none");
}

#[test]
fn intx_serves_a_function_without_msix() {
    let _dev = bench(8, 2048, 1);
    offer_irq(false, true);
    let driver = setup::run().expect("the device comes up on its line");
    assert_eq!(binds(), [Event::IrqBind { msix: true }, Event::IrqBind { msix: false }]);
    assert_eq!(driver.capacity_sectors, 2048, "the config read where MSI-X off puts it");
}

#[test]
fn a_function_with_neither_is_refused_by_name_and_given_back() {
    let _dev = bench(8, 2048, 0);
    offer_irq(false, true);
    let refused = setup::run().err();
    assert_eq!(refused, Some("virtio-blk: no MSI-X vector and no routed INTx line"));
    assert_eq!(events().last(), Some(&Event::Release));
}

#[test]
fn a_legacy_ring_of_1024_is_laid_out_in_a_grant_sized_for_it() {
    let dev = bench(1024, 2048, 1);
    dev.borrow_mut().msix = true;
    setup::run().expect("QEMU's queue-size=1024 comes up");
    let ring = ring_bytes(1024).expect("a power of two") as u64;
    assert!(events().contains(&Event::DmaMap(ring)), "{:?}", events());
}
