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

//! What firmware describes and what the hypervisor offers: the device facts
//! the ACPI tables give, the ACPI owner's calls, the VMBus core's, and the
//! boot framebuffer's.

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        SyscallNumber::MkAcpiDeviceName
        | SyscallNumber::MkUsbPortFacts
        | SyscallNumber::MkNumaNodeOf
        | SyscallNumber::MkNumaDistance
        | SyscallNumber::MkDeviceNode
        | SyscallNumber::MkNumaHmat => caps.can_device_enum(),
        /*
         * Firmware's AML reaches memory and ports no claim covers, so until the
         * owner names a bit of its own the ACPI calls are Admin's alone.
         */
        SyscallNumber::MkAcpiTable
        | SyscallNumber::MkAcpiRegion
        | SyscallNumber::MkAcpiPort
        | SyscallNumber::MkAcpiPci
        | SyscallNumber::MkAcpiEvent
        | SyscallNumber::MkAcpiSleep
        | SyscallNumber::MkSmbiosTable
        | SyscallNumber::MkAcpiCpu
        | SyscallNumber::MkAcpiHolds
        | SyscallNumber::MkPciProbeFunction
        | SyscallNumber::MkPciDepart
        | SyscallNumber::MkAcpiFunctionOf
        | SyscallNumber::MkAcpiHwError
        | SyscallNumber::MkAcpiPlatformDevice
        | SyscallNumber::MkAcpiUsbPorts
        | SyscallNumber::MkAcpiExternalPort
        | SyscallNumber::MkAcpiReserved
        | SyscallNumber::MkAcpiProximity
        | SyscallNumber::MkAcpiOscGrant
        | SyscallNumber::MkAcpiFirstPass
        | SyscallNumber::MkAcpiSlot
        | SyscallNumber::MkAcpiLoaded
        | SyscallNumber::MkAcpiThermalLimit
        | SyscallNumber::MkAcpiLpit
        | SyscallNumber::MkAcpiPower => caps.is_valid() && caps.is_admin(),

        /* VMBus through the SynIC; each also checks the VMBus root's claim, or for
         * events and signals a channel record's. */
        SyscallNumber::MkHvPost
        | SyscallNumber::MkHvChannel
        | SyscallNumber::MkHvRecv
        | SyscallNumber::MkHvEvents
        | SyscallNumber::MkHvSignal => caps.can_driver(),
        /* The boot framebuffer's geometry, to the holder of its claim (L31). */
        SyscallNumber::MkBootFramebuffer => caps.can_mmio(),

        _ => return None,
    })
}
