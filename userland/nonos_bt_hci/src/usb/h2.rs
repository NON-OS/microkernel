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

//! HCI over USB (Vol 4, Part B): commands as control transfers, events
//! from the interrupt endpoint, ACL data on the bulk pair. Each poll arms
//! one endpoint packet, so a transfer never holds more than the device
//! may send at once and the joiner does the rest.

use alloc::vec;
use alloc::vec::Vec;

use super::descriptor::BtInterface;
use super::joiner::Joiner;
use super::pipes::UsbPipes;
use crate::error::HciError;

/// The errno for a bulk OUT the device took only part of.
pub(super) const E_IO: i32 = -5;

pub(super) type Polled = Result<Option<Vec<u8>>, HciError>;

pub struct UsbHci<P> {
    pub(super) pipes: P,
    pub(super) events: (Joiner, Vec<u8>),
    pub(super) acl: (Joiner, Vec<u8>),
}

impl<P: UsbPipes> UsbHci<P> {
    /// ACL packets up to `acl_data_max` data octets are accepted; once
    /// bring-up has read the controller's own bound, `set_acl_max` sets it.
    pub fn new(pipes: P, iface: &BtInterface, acl_data_max: u16) -> Self {
        let ev = vec![0; usize::from(iface.events.max_packet)];
        let acl = vec![0; usize::from(iface.acl_in.max_packet)];
        Self { pipes, events: (Joiner::events(), ev), acl: (Joiner::acl(acl_data_max), acl) }
    }

    pub fn set_acl_max(&mut self, acl_data_max: u16) {
        self.acl.0 = Joiner::acl(acl_data_max);
    }

    pub fn pipes(&mut self) -> &mut P {
        &mut self.pipes
    }
}

/// A whole packet already joined, else one transfer's worth more.
pub(super) fn take(j: &mut (Joiner, Vec<u8>), got: Result<Option<usize>, i32>) -> Polled {
    if let Some(n) = got.map_err(HciError::Transport)? {
        j.0.push(&j.1[..n.min(j.1.len())]);
    }
    j.0.take()
}
