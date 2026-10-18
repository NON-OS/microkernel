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

//! The driver broker, the store and data volume, and foreign code.

use crate::capabilities::CapabilityToken;
use crate::syscall::numbers::SyscallNumber;

pub(super) fn check(caps: &CapabilityToken, number: SyscallNumber) -> Option<bool> {
    Some(match number {
        SyscallNumber::MkDeviceList => caps.can_device_enum(),
        SyscallNumber::MkDeviceClaim | SyscallNumber::MkDeviceRelease => caps.can_driver(),
        SyscallNumber::MkMmioMap | SyscallNumber::MkMmioUnmap => caps.can_mmio(),
        SyscallNumber::MkIrqBind
        | SyscallNumber::MkIrqUnbind
        | SyscallNumber::MkIrqAck
        | SyscallNumber::MkIrqPoll => caps.can_irq(),
        SyscallNumber::MkIrqWait => caps.can_irq(),
        SyscallNumber::MkDmaMap | SyscallNumber::MkDmaUnmap => caps.can_dma(),
        SyscallNumber::MkPciConfigRead | SyscallNumber::MkPciConfigWrite => caps.can_driver(),
        SyscallNumber::MkPioGrant
        | SyscallNumber::MkPioRead
        | SyscallNumber::MkPioWrite
        | SyscallNumber::MkPioRelease => caps.can_pio(),

        SyscallNumber::MkDebug => caps.can_debug(),
        SyscallNumber::MkStdoutWrite | SyscallNumber::MkPrivateWrite => caps.can_ipc(),
        SyscallNumber::MkStoreWrite | SyscallNumber::MkStoreRead => caps.can_store_write(),
        SyscallNumber::MkDataImport => caps.can_store_write() && caps.can_open_files(),
        SyscallNumber::MkDataPassphrase => caps.can_store_write() && caps.can_open_files(),
        SyscallNumber::MkDataStat | SyscallNumber::MkDataRead => caps.can_open_files(),
        SyscallNumber::MkDataFeedBegin
        | SyscallNumber::MkDataFeed
        | SyscallNumber::MkDataRemove => caps.can_stream_import(),

        /*
         * Hosting unverified code is one right, and it covers every call
         * that touches a guest: creating it, building its address space,
         * and answering for it.
         */
        SyscallNumber::MkForeignSpawn
        | SyscallNumber::MkForeignStart
        | SyscallNumber::MkForeignWait
        | SyscallNumber::MkForeignReply
        | SyscallNumber::MkForeignContext
        | SyscallNumber::MkForeignSignal
        | SyscallNumber::MkForeignInterrupt
        | SyscallNumber::MkPeerMap
        | SyscallNumber::MkPeerCopy
        | SyscallNumber::MkPeerProtect
        | SyscallNumber::MkForeignThread
        | SyscallNumber::MkPeerTls
        | SyscallNumber::MkForeignFork
        | SyscallNumber::MkPeerUnmap
        | SyscallNumber::MkForeignExec => caps.can_foreign_exec(),

        _ => return None,
    })
}
