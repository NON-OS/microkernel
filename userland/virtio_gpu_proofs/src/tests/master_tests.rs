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

//! Bus mastering at bring-up. A device a dead predecessor left
//! live still holds its rings' addresses, in frames since freed, and runs
//! its queue once it may reach memory: Bus Master goes on only after the
//! device was reset and read back reset and its queue moved to this
//! driver's grant, and DRIVER_OK only after that. A device refused at any
//! step, its features or its queue, never gets it.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use nonos_libc::{on_pci_write, pci_writes, MK_PCI_CFG_COMMAND};
use nonos_virtio::common::{DEVICE_FEATURE, DEVICE_STATUS, QUEUE_DESC, QUEUE_NOTIFY_OFF};
use nonos_virtio::common::{QUEUE_SIZE as SIZE, STATUS_DRIVER_OK, STATUS_FAILED};

use super::harness::HANDSHAKE;
use super::predecessor::{left_live, window};
use crate::setup::{pci, transport};

#[test]
fn a_device_left_live_is_seen_reset_and_on_this_grant_before_it_may_master_the_bus() {
    let bar = left_live();
    let (at_master, b) = (Rc::new(Cell::new((0, 0))), Arc::clone(&bar));
    let seen = Rc::clone(&at_master);
    on_pci_write(move |_| seen.set((b.wrote8(DEVICE_STATUS), b.wrote32(QUEUE_DESC))));
    let ready = transport::start(1, 1, &window(&bar)).expect("a bring-up");
    assert_eq!(pci_writes(), [(MK_PCI_CFG_COMMAND, pci::MASTER)]);
    let (status, ring) = at_master.get();
    assert_eq!(status, HANDSHAKE & !STATUS_DRIVER_OK, "Bus Master before the reset was seen");
    assert_eq!(
        ring, ready.control_queue.layout.region_device_addr as u32,
        "Bus Master with the rings still stale"
    );
    assert_eq!(bar.wrote8(DEVICE_STATUS), HANDSHAKE, "DRIVER_OK once the device may reach memory");
}

#[test]
fn a_device_refused_at_any_step_never_masters_the_bus() {
    // No VERSION_1, then no control queue, a ring of one, a doorbell past
    // the notify region.
    let refusals = [(DEVICE_FEATURE, 0), (SIZE, 0), (SIZE, 1), (QUEUE_NOTIFY_OFF, 0x100)];
    for (register, value) in refusals {
        let bar = left_live();
        bar.present16(register, value);
        assert!(transport::start(1, 1, &window(&bar)).is_err(), "{register:#x} = {value:#x}");
        assert_eq!(pci_writes(), [], "Bus Master on a refusal path: {register:#x} = {value:#x}");
        assert_eq!(bar.wrote8(DEVICE_STATUS) & STATUS_FAILED, STATUS_FAILED, "not left FAILED");
    }
}
