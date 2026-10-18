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

//! The kinds `MkInstallSource` serves that are records the kernel holds,
//! not modules the loader left: the release's approval of the kernel, and
//! the disk this boot's block layer chose. Each is read like an image.

use super::errnos::{ERRNO_FAULT, ERRNO_NOENT};
use super::install_source_in_use::{in_use_record, AHCI, NVME, USB_MSC, VIRTIO_BLK};
use super::install_source_kinds::{KIND_IN_USE_DISK, KIND_KERNEL_APPROVAL};
use crate::hardware::block_device::{chosen, Kind};

/// What `kind` serves when it is such a record, None when it is not one.
pub(super) fn record(kind: u64, offset: u64, out_ptr: u64, out_len: u64) -> Option<i64> {
    if kind == u64::from(KIND_KERNEL_APPROVAL) {
        return Some(super::install_source_approval::approval(offset, out_ptr, out_len));
    }
    (kind == u64::from(KIND_IN_USE_DISK)).then(|| in_use(offset, out_ptr, out_len))
}

/* Only once a disk carried the layout is one kept (select.rs); before
 * that no disk is in use, which is no such record. The kernel's block
 * layer speaks to each backend's instance 0. */
fn in_use(offset: u64, out_ptr: u64, out_len: u64) -> i64 {
    let Some(backend) = chosen() else {
        return ERRNO_NOENT;
    };
    let code = match backend.kind() {
        Kind::VirtioBlk => VIRTIO_BLK,
        Kind::Ahci => AHCI,
        Kind::Nvme => NVME,
        Kind::UsbMsc => USB_MSC,
        // The record has no code for a disk on the block wire.
        Kind::Wire => return ERRNO_NOENT,
    };
    serve(&in_use_record(code, 0), offset, out_ptr, out_len)
}

/// `all` from `offset`, at most `out_len` bytes; zero past the end.
pub(super) fn serve(all: &[u8], offset: u64, out_ptr: u64, out_len: u64) -> i64 {
    let start = usize::try_from(offset).unwrap_or(usize::MAX).min(all.len());
    let end = all.len().min(start.saturating_add(usize::try_from(out_len).unwrap_or(usize::MAX)));
    if end == start {
        return 0;
    }
    if crate::usercopy::write_user_bytes(out_ptr, &all[start..end]).is_err() {
        return ERRNO_FAULT;
    }
    (end - start) as i64
}
