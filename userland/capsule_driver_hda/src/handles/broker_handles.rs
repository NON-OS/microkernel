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

use nonos_libc::MmioMapOut;

/// What a bring-up holds from the broker from the moment the register window
/// is mapped. Dropping it is the one way the controller is given up.
pub struct BrokerHandles {
    pub(super) device_id: u64,
    pub(super) claim_epoch: u64,
    pub(super) mmio_grant_id: u64,
    pub(super) mmio_user_va: u64,
    pub(super) mmio_len: u64,
    pub(super) irq_grant_id: u64,
}

impl BrokerHandles {
    pub(crate) const fn new(
        device_id: u64,
        claim_epoch: u64,
        mmio: &MmioMapOut,
        irq_grant_id: u64,
    ) -> Self {
        let (mmio_grant_id, mmio_user_va, mmio_len) = (mmio.grant_id, mmio.user_va, mmio.length);
        Self { device_id, claim_epoch, mmio_grant_id, mmio_user_va, mmio_len, irq_grant_id }
    }
}
