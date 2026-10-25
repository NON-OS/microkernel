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

//! The release's approval of the kernel, served by `MkInstallSource`.

use super::errnos::ERRNO_NOENT;

/// The approval the loader handed over, read like an image: the bytes from
/// `offset`, zero past the end. None handed over is no such image.
pub(super) fn approval(offset: u64, out_ptr: u64, out_len: u64) -> i64 {
    let Some(h) = crate::boot::handoff::get_handoff() else {
        return ERRNO_NOENT;
    };
    if h.policy.approval_present != 1 {
        return ERRNO_NOENT;
    }
    super::install_source_records::serve(&h.policy.approval[..], offset, out_ptr, out_len)
}
