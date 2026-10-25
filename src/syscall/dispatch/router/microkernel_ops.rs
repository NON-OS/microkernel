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
pub(super) use super::microkernel_handle::handle;
use crate::syscall::numbers::SyscallNumber::{self, *};
pub(super) fn matches(nr: SyscallNumber) -> bool {
    matches!(
        nr,
        MkIpcSend
            | MkIpcRecv
            | MkIpcCall
            | MkIpcRecvFrom
            | MkIpcReply
            | MkIpcSendToPid
            | MkServiceLookup
            | MkThreadGroup
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
            | MkLogTail
            | MkAttestPolicy
            | MkBootAttest
            | MkDeviceSecret
            | MkBootSlots
            | MkEnroll
            | MkInstallSource
            | MkDevRootRequest
            | MkDevRootConfirm
            | MkCapGrant
            | MkCapRevoke
            | MkCapCheck
            | MkDeviceList
            | MkDeviceClaim
            | MkDeviceRelease
            | MkBootFramebuffer
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
            | MkHvBackdoor
            | MkHvPost
            | MkHvRecv
            | MkHvEvents
            | MkHvSignal
            | MkHvChannel
            | MkAcpiTable
            | MkAcpiRegion
            | MkAcpiPort
            | MkAcpiPci
            | MkAcpiPower
            | MkAcpiEvent
            | MkAcpiSleep
            | MkSmbiosTable
            | MkAcpiCpu
            | MkAcpiHolds
            | MkPciProbeFunction
            | MkPciDepart
            | MkAcpiFunctionOf
            | MkAcpiHwError
            | MkAcpiPlatformDevice
            | MkAcpiDeviceName
            | MkAcpiUsbPorts
            | MkUsbPortFacts
            | MkAcpiExternalPort
            | MkAcpiReserved
            | MkAcpiProximity
            | MkNumaNodeOf
            | MkNumaDistance
            | MkDeviceNode
            | MkAcpiOscGrant
            | MkAcpiFirstPass
            | MkAcpiSlot
            | MkAcpiMeasured
            | MkAcpiLoaded
            | MkAcpiThermalLimit
            | MkNumaHmat
            | MkAcpiLpit
            | MkBootLogo
            | MkDebug
            | MkStdoutWrite
            | MkPrivateWrite
            | MkStoreWrite
            | MkStoreRead
            | MkDataImport
            | MkDataStat
            | MkDataRead
            | MkDataPassphrase
            | MkDataFeedBegin
            | MkDataFeed
            | MkDataRemove
            | MkSpawnInstance
            | MkForeignSpawn
            | MkForeignStart
            | MkForeignWait
            | MkForeignReply
            | MkForeignContext
            | MkForeignSignal
            | MkForeignInterrupt
            | MkPeerMap
            | MkPeerCopy
            | MkPeerProtect
            | MkForeignThread
            | MkPeerTls
            | MkForeignFork
            | MkPeerUnmap
            | MkForeignExec
            | MkLinuxRunNet
            | MkSealRead
            | MkSealNote
            | MkProveJob
            | MkLocalSign
            | MkLocalVerify
            | MkAppInstall
            | MkDevRootLocal
            | MkLocalConsent
            | MkLocalRestore
            | MkAppLaunch
            | MkAppInstallStatus
            | MkAppUninstall
            | MkToolRun
            | MkCapsOf
            | MkTtySet
            | MkTtyQuery
    )
}
