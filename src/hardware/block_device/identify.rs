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

//! Whether one backend's disk is the NONOS disk.

use super::seen::Seen;
use super::BlockDeviceError;
use super::{backend::Backend, capacity::capacity_on, fit::sectors_fit, read::read_on};
use crate::fs::blockfs_volume::PLAN_LBA;

/// The package store's first sector and header magic, as vfs writes them.
const STORE_LBA: u64 = 256;
const STORE_MAGIC: &[u8; 8] = b"NONOSTR1";
/// The disk plan's magic, as `blockfs_volume` reads it at `PLAN_LBA`.
const PLAN_MAGIC: &[u8; 8] = b"NONOSDP1";

pub(super) enum Found {
    /// The disk carries the store header or the disk plan.
    Layout,
    /// No driver, no disk, or a disk without either structure.
    Absent,
    /// The backend could not be asked now; the answer may differ later.
    Refused(BlockDeviceError),
}

/// Whether `backend`'s disk is the NONOS disk, and what was seen on it,
/// which select.rs says on the log.
pub(super) fn identify(backend: Backend) -> (Found, Seen) {
    let sectors = match capacity_on(backend) {
        Ok(s) => s,
        Err(e) => return (classify(e), Seen::NoSize(e)),
    };
    if sectors <= STORE_LBA {
        return (Found::Absent, Seen::TooSmall(sectors));
    }
    if !sectors_fit(backend) {
        return (Found::Absent, Seen::NotFiveTwelve(sectors));
    }
    let store = match has_magic(backend, STORE_LBA, STORE_MAGIC) {
        (Found::Absent, got) => got,
        (found, got) => return (found, Seen::At { sectors, lba: STORE_LBA, got, plan: None }),
    };
    if sectors <= PLAN_LBA {
        return (Found::Absent, Seen::At { sectors, lba: STORE_LBA, got: store, plan: None });
    }
    let (found, plan) = has_magic(backend, PLAN_LBA, PLAN_MAGIC);
    (found, Seen::At { sectors, lba: STORE_LBA, got: store, plan: Some(plan) })
}

/// Whether the sector at `lba` starts with `magic`, and its first eight
/// bytes or the read's error.
fn has_magic(backend: Backend, lba: u64, magic: &[u8; 8]) -> (Found, Result<[u8; 8], BlockDeviceError>) {
    let mut sector = [0u8; 512];
    match read_on(backend, lba, &mut sector) {
        Ok(()) => {
            let mut head = [0u8; 8];
            head.copy_from_slice(&sector[..8]);
            let found = if &head == magic { Found::Layout } else { Found::Absent };
            (found, Ok(head))
        }
        Err(e) => (classify(e), Err(e)),
    }
}

/// A stopped driver or a disk that cannot serve the sector is not this disk;
/// a refused caller or a lost reply says nothing about the disk.
fn classify(e: BlockDeviceError) -> Found {
    match e {
        BlockDeviceError::Stale
        | BlockDeviceError::AccessDenied
        | BlockDeviceError::NoCallerPid
        | BlockDeviceError::TransportFailure
        | BlockDeviceError::ProtocolMismatch
        | BlockDeviceError::NotReady => Found::Refused(e),
        _ => Found::Absent,
    }
}
