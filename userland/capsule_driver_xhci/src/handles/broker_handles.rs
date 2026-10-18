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
/// What the broker granted for one controller: its claim, its register
/// window and its interrupt. Given back once, by `release` or on drop.
#[derive(Debug)]
pub struct BrokerHandles {
    pub(super) device_id: u64,
    pub(super) claim_epoch: u64,
    pub(super) mmio_grant_id: u64,
    pub(super) mmio_user_va: u64,
    pub(super) irq_grant_id: u64,
    pub(super) released: bool,
}
impl BrokerHandles {
    pub(crate) fn new(
        device_id: u64,
        claim_epoch: u64,
        mmio_grant_id: u64,
        mmio_user_va: u64,
        irq_grant_id: u64,
    ) -> Self {
        Self { device_id, claim_epoch, mmio_grant_id, mmio_user_va, irq_grant_id, released: false }
    }
}
