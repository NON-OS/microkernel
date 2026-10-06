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

//! Syscall numbers, spelt as the four letters `src/syscall/abi/registry`
//! gives each, so an attempt names its call the way the ABI table does.

const fn code(tag: &[u8; 4]) -> i64 {
    u32::from_le_bytes(*tag) as i64
}

pub const MSVL: i64 = code(b"MSVL"); // MkServiceLookup
pub const MISD: i64 = code(b"MISD"); // MkIpcSend
pub const MISP: i64 = code(b"MISP"); // MkIpcSendToPid
pub const MMMP: i64 = code(b"MMMP"); // MkMmioMap
pub const MSVR: i64 = code(b"MSVR"); // MkServiceRegister
pub const MMAP: i64 = code(b"MMAP"); // MkMmap
pub const MUMP: i64 = code(b"MUMP"); // MkMunmap
pub const MDBG: i64 = code(b"MDBG"); // MkDebug
pub const MPST: i64 = code(b"MPST"); // MkProcStat
pub const MAST: i64 = code(b"MAST"); // MkAttestStatus
pub const MKIL: i64 = code(b"MKIL"); // MkKill
pub const MICL: i64 = code(b"MICL"); // MkIpcCall

/// Each call a capability gates, the capability, and what it would let an
/// attacker do. The capsule holds none of these capabilities.
pub const GATED: [(i64, &[u8]); 20] = [
    (code(b"MDCL"), b"claim a device without Driver, MkDeviceClaim"),
    (code(b"MDMM"), b"map a DMA buffer without Dma, MkDmaMap"),
    (code(b"MIRB"), b"bind a device interrupt without Irq, MkIrqBind"),
    (code(b"MPCR"), b"read PCI config space without Driver, MkPciConfigRead"),
    (code(b"MPGT"), b"take an I/O port range without Pio, MkPioGrant"),
    (code(b"MAEN"), b"read every capsule's measurement without AttestRead, MkAttestEntries"),
    (code(b"MADC"), b"ask the TPM to sign an attestation without AttestRead, MkAttestDoc"),
    (code(b"MDVS"), b"read the device secret without DeviceSecret, MkDeviceSecret"),
    (code(b"MENR"), b"read the TPM identity keys without DeviceSecret, MkEnroll"),
    (code(b"MCGT"), b"grant itself Network without Admin, MkCapGrant"),
    (code(b"MTAD"), b"set the clock without TimeSet, MkTimeAdjust"),
    (code(b"MSWR"), b"write the capsule store without StoreWrite, MkStoreWrite"),
    (code(b"MSPI"), b"open another app's window without SpawnWindow, MkSpawnInstance"),
    (code(b"MAIN"), b"install an app without AppInstall, MkAppInstall"),
    (code(b"MFSP"), b"host a foreign guest without ForeignExec, MkForeignSpawn"),
    (code(b"MPCP"), b"copy another process's memory without ForeignExec, MkPeerCopy"),
    (code(b"MLSG"), b"mint a signature trailer without LocalSign, MkLocalSign"),
    (code(b"MDRO"), b"enrol a signing root without EnrolDevRoot, MkDevRootLocal"),
    (code(b"MIEP"), b"inject input events without InputSource, MkInputEventPost"),
    (code(b"MIED"), b"drain every keystroke without InputSource, MkInputEventDrain"),
];
