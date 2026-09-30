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

use crate::syscall::numbers::SyscallNumber;

pub(super) use super::microkernel_handle::handle;

pub(super) fn matches(nr: SyscallNumber) -> bool {
    use SyscallNumber::*;
    matches!(
        nr,
        MkIpcSend
            | MkIpcRecv
            | MkIpcCall
            | MkIpcRecvFrom
            | MkIpcReply
            | MkIpcSendToPid
            | MkServiceLookup
            | MkServiceRegister
            | MkMmap
            | MkMunmap
            | MkCapsuleLoad
            | MkCapsuleVerify
            | MkExit
            | MkPidAlive
            | MkWait
            | MkKill
            | MkGetPid
            | MkArgs
            | MkThreadSpawn
            | MkSetTls
            | MkYield
            | MkTimeMillis
            | MkTimeMonotonic
            | MkTimeRtc
            | MkTimeAdjust
            | MkBatteryStatus
            | MkProcStat
            | MkFutexWait
            | MkFutexWake
            | MkProcOutput
            | MkProcInput
            | MkStdinRead
            | MkAttestStatus
            | MkAttestDoc
            | MkAttestEntries
            | MkInstallSource
            | MkDevRootRequest
            | MkDevRootConfirm
            | MkCapGrant
            | MkCapRevoke
            | MkCapCheck
            | MkDeviceList
            | MkDeviceClaim
            | MkDeviceRelease
            | MkMmioMap
            | MkMmioUnmap
            | MkIrqBind
            | MkIrqUnbind
            | MkIrqAck
            | MkIrqPoll
            | MkIrqWait
            | MkDmaMap
            | MkDmaUnmap
            | MkPciConfigRead
            | MkPciConfigWrite
            | MkPioGrant
            | MkPioRead
            | MkPioWrite
            | MkPioRelease
            | MkDebug
            | MkStdoutWrite
            | MkPrivateWrite
            | MkStoreWrite
            | MkStoreRead
            | MkDataImport
            | MkDataStat
            | MkDataRead
            | MkDataPassphrase
            | MkSpawnInstance
            | MkForeignSpawn
            | MkForeignStart
            | MkForeignWait
            | MkForeignReply
            | MkForeignContext
            | MkForeignSignal
            | MkPeerMap
            | MkPeerCopy
            | MkPeerProtect
            | MkForeignThread
            | MkPeerTls
            | MkForeignFork
            | MkPeerUnmap
            | MkForeignExec
            | MkLocalSign
            | MkLocalVerify
            | MkAppInstall
            | MkDevRootLocal
            | MkLocalConsent
            | MkLocalRestore
            | MkAppLaunch
            | MkAppInstallStatus
            | MkToolRun
            | MkTtySet
            | MkTtyQuery
    )
}
