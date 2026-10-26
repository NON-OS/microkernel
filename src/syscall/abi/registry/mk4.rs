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

//! The Mk* syscalls, part 4 of 4, in the registry's order (mk.rs).

use super::mk::e;
use crate::syscall::abi::AbiEntry;
use crate::syscall::numbers::SyscallNumber;

pub(super) const ENTRIES: &[AbiEntry] = &[
    e(b"MDST", SyscallNumber::MkDataStat, "MkDataStat"),
    e(b"MDRD", SyscallNumber::MkDataRead, "MkDataRead"),
    e(b"MDPW", SyscallNumber::MkDataPassphrase, "MkDataPassphrase"),
    e(b"MDFB", SyscallNumber::MkDataFeedBegin, "MkDataFeedBegin"),
    e(b"MDFD", SyscallNumber::MkDataFeed, "MkDataFeed"),
    e(b"MDRM", SyscallNumber::MkDataRemove, "MkDataRemove"),
    e(b"MCVF", SyscallNumber::MkCapsuleVerify, "MkCapsuleVerify"),
    e(b"MTBL", SyscallNumber::MkAcpiTable, "MkAcpiTable"),
    e(b"MRGN", SyscallNumber::MkAcpiRegion, "MkAcpiRegion"),
    e(b"MRIO", SyscallNumber::MkAcpiPort, "MkAcpiPort"),
    e(b"MRPC", SyscallNumber::MkAcpiPci, "MkAcpiPci"),
    e(b"MAPW", SyscallNumber::MkAcpiPower, "MkAcpiPower"),
    e(b"MAEV", SyscallNumber::MkAcpiEvent, "MkAcpiEvent"),
    e(b"MASL", SyscallNumber::MkAcpiSleep, "MkAcpiSleep"),
    e(b"MSMB", SyscallNumber::MkSmbiosTable, "MkSmbiosTable"),
    e(b"MACP", SyscallNumber::MkAcpiCpu, "MkAcpiCpu"),
    e(b"MAHO", SyscallNumber::MkAcpiHolds, "MkAcpiHolds"),
    e(b"MPPF", SyscallNumber::MkPciProbeFunction, "MkPciProbeFunction"),
    e(b"MPDP", SyscallNumber::MkPciDepart, "MkPciDepart"),
    e(b"MAFO", SyscallNumber::MkAcpiFunctionOf, "MkAcpiFunctionOf"),
    e(b"MAHE", SyscallNumber::MkAcpiHwError, "MkAcpiHwError"),
    e(b"MAPD", SyscallNumber::MkAcpiPlatformDevice, "MkAcpiPlatformDevice"),
    e(b"MADN", SyscallNumber::MkAcpiDeviceName, "MkAcpiDeviceName"),
    e(b"MAUP", SyscallNumber::MkAcpiUsbPorts, "MkAcpiUsbPorts"),
    e(b"MUPF", SyscallNumber::MkUsbPortFacts, "MkUsbPortFacts"),
    e(b"MAXP", SyscallNumber::MkAcpiExternalPort, "MkAcpiExternalPort"),
    e(b"MAMR", SyscallNumber::MkAcpiReserved, "MkAcpiReserved"),
    e(b"MAPX", SyscallNumber::MkAcpiProximity, "MkAcpiProximity"),
    e(b"MNNO", SyscallNumber::MkNumaNodeOf, "MkNumaNodeOf"),
    e(b"MNDS", SyscallNumber::MkNumaDistance, "MkNumaDistance"),
    e(b"MDNN", SyscallNumber::MkDeviceNode, "MkDeviceNode"),
    e(b"MAOG", SyscallNumber::MkAcpiOscGrant, "MkAcpiOscGrant"),
    e(b"MNHM", SyscallNumber::MkNumaHmat, "MkNumaHmat"),
    e(b"MALP", SyscallNumber::MkAcpiLpit, "MkAcpiLpit"),
    e(b"MBGL", SyscallNumber::MkBootLogo, "MkBootLogo"),
    e(b"MAFP", SyscallNumber::MkAcpiFirstPass, "MkAcpiFirstPass"),
    e(b"MRSL", SyscallNumber::MkAcpiSlot, "MkAcpiSlot"),
    e(b"MAMT", SyscallNumber::MkAcpiMeasured, "MkAcpiMeasured"),
    e(b"MALD", SyscallNumber::MkAcpiLoaded, "MkAcpiLoaded"),
    e(b"MATL", SyscallNumber::MkAcpiThermalLimit, "MkAcpiThermalLimit"),
];
