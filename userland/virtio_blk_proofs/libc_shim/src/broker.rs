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

//! The claim and the PCI config space. The config space reads as the test
//! presents it, zeros by default: a function with no capability list, which
//! the transport choice leaves to its legacy registers.

use std::cell::RefCell;

use crate::consts::MK_PCI_CFG_COMMAND;
use crate::events::{record, Event};

thread_local! {
    static CONFIG: RefCell<[u32; 64]> = const { RefCell::new([0; 64]) };
}

/// What config space reads, one dword per entry from offset zero.
pub fn present_config(words: &[u32]) {
    CONFIG.with(|c| {
        let mut c = c.borrow_mut();
        c.iter_mut().zip(words).for_each(|(slot, w)| *slot = *w);
    });
}

/// Every claim is granted, at epoch 1.
pub fn mk_device_claim(_device_id: u64) -> i64 {
    record(Event::Claim);
    1
}

pub fn mk_device_release(_device_id: u64) -> i64 {
    record(Event::Release);
    0
}

pub fn mk_pci_config_read(_device_id: u64, _epoch: u64, offset: u32, _width: u32) -> i64 {
    CONFIG.with(|c| c.borrow().get(offset as usize / 4).map_or(-22, |w| i64::from(*w)))
}

pub fn mk_pci_config_write(_device_id: u64, _epoch: u64, offset: u32, value: u16) -> i64 {
    if offset == MK_PCI_CFG_COMMAND {
        record(Event::Command(value));
    }
    0
}
