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

use super::vendor::Hypervisor;

/// The vendor, and whether the guest also sees the Hyper-V interface ("Hv#1"
/// at 0x40000001). KVM, Xen and VirtualBox all offer that interface on
/// request, so the VMBus capsules key on the interface, not on the vendor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guest {
    pub vendor: Hypervisor,
    pub hyperv_abi: bool,
}
