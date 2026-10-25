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

//! A device the firmware description names, confined through its `iommus`
//! (the C-42): every stream in its capsule's domain, or the claim is
//! refused. Never left unconfined while the description puts it behind a
//! unit; without `iommus` it is unconfined as before, and said.

use super::detach::detach;
use super::one::attach_one;
use super::refuse::Refused;
use super::say::say;
use super::verdict::{verdict, Verdict};
use crate::hardware::broker::described::DescribedIommus;
use crate::memory::iommu::{translates, DeviceAddress};

pub(super) fn attach_streams(
    pid: u32,
    device_id: u64,
    iommus: DescribedIommus,
    port_only: bool,
) -> Result<(), Refused> {
    let in_service = crate::arch::iommu_unit_in_service;
    match verdict(&iommus, in_service, |sid| translates(DeviceAddress::stream(sid))) {
        Verdict::Unconfined => {
            say(b"unconfined: no iommus, so no DMA grant but under the opt-in");
            Ok(())
        }
        Verdict::Refuse(why) => {
            say(why);
            Err(Refused)
        }
        Verdict::Attach => {
            for sid in iommus.sids() {
                let stream = DeviceAddress::stream(*sid);
                if attach_one(pid, device_id, stream, (true, port_only), (&[], &[])).is_err() {
                    // The streams already in go back out: all of them or none.
                    detach(pid, device_id);
                    return Err(Refused);
                }
            }
            Ok(())
        }
    }
}
