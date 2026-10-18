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

//! Names the hypervisor in the boot log, once the device table exists.

/// Report which hypervisor, if any, the kernel runs under.
///
/// x86_64 reads the CPUID hypervisor leaves and settles what they leave open
/// from the PCI table. The other architectures have no such leaf; their
/// firmware describes a virtual machine in the device tree instead.
#[cfg(target_arch = "x86_64")]
pub(crate) fn report_hypervisor() {
    crate::arch::x86_64::hypervisor::report();
}

#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn report_hypervisor() {}
