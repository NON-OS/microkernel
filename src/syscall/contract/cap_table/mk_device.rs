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
        // A device reading another capsule's surface: the driver's DMA right
        // and the surface right its attach already took.
        SyscallNumber::MkDmaImportSurface | SyscallNumber::MkDmaUnimportSurface => {
            caps.can_dma() && caps.can_surface_map()
        }
        SyscallNumber::MkPciConfigRead | SyscallNumber::MkPciConfigWrite => caps.can_driver(),
        SyscallNumber::MkPioGrant
        | SyscallNumber::MkPioRead
        | SyscallNumber::MkPioWrite
        | SyscallNumber::MkPioRelease
        | SyscallNumber::MkHvBackdoor => caps.can_pio(),

        SyscallNumber::MkDebug => caps.can_debug(),
        SyscallNumber::MkStdoutWrite | SyscallNumber::MkPrivateWrite => caps.can_ipc(),
        SyscallNumber::MkStoreWrite | SyscallNumber::MkStoreRead => caps.can_store_write(),
        SyscallNumber::MkDataImport
        | SyscallNumber::MkDataPassphrase
        | SyscallNumber::MkDataHandover => caps.can_store_write() && caps.can_open_files(),
        SyscallNumber::MkDataStat | SyscallNumber::MkDataRead => caps.can_open_files(),
        SyscallNumber::MkDataFeedBegin
        | SyscallNumber::MkDataFeed
        | SyscallNumber::MkDataRemove => caps.can_stream_import(),
        /* The person's files' objects: the user-files broker's alone. */
        SyscallNumber::MkUserObjPut
        | SyscallNumber::MkUserObjRead
        | SyscallNumber::MkUserObjSize
        | SyscallNumber::MkUserObjDrop => caps.can_user_files(),

        /*
         * Hosting unverified code is one right: every call that touches a
         * guest, creating it, building its address space, answering for it.
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
        | SyscallNumber::MkPeerReadRows
        | SyscallNumber::MkForeignThread
        | SyscallNumber::MkPeerTls
        | SyscallNumber::MkForeignFork
        | SyscallNumber::MkPeerUnmap
        | SyscallNumber::MkForeignExec => caps.can_foreign_exec(),
        /*
         * A pin makes a surface of a guest's pages: both rights, as either
         * alone hosts no guest or makes no surface.
         */
        SyscallNumber::MkPeerPin | SyscallNumber::MkPeerUnpin => {
            caps.can_foreign_exec() && caps.can_surface_create()
        }
        /* Their handlers admit only the Linux run role: its own pid; a SERVES pair. */
        SyscallNumber::MkLinuxRunNet | SyscallNumber::MkServeOne => caps.can_foreign_exec(),

        _ => return None,
    })
}
