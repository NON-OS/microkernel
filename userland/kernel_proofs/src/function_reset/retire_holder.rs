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

//! A holder of one claimed device, every surface of it still live, for
//! retire_fake's steps to take down.

use super::types::Claim;

pub(super) const PID: u32 = 7;

#[derive(Default)]
pub(super) struct Holder {
    pub(super) log: Vec<&'static str>,
    pub(super) claims: Vec<Claim>,
    pub(super) busy: Vec<u64>,
    pub(super) granted: (u32, u32),
    pub(super) taken: (u32, u32),
    pub(super) mapped: bool,
    pub(super) ports_running: bool,
    pub(super) in_iommu: bool,
    pub(super) freed: u32,
    pub(super) fault_during_reset: bool,
}

pub(super) fn holding() -> Holder {
    let claims = vec![Claim { pid: PID, device_id: 1, epoch: 3 }];
    let (mapped, ports_running, in_iommu) = (true, true, true);
    Holder { claims, granted: (2, 4), mapped, ports_running, in_iommu, ..Holder::default() }
}
