// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The release's approval of this kernel, read from beside it.
//!
//! It is a signature over the kernel's PCR 9 policy, so it cannot live inside
//! the image it approves. The loader reads it and hands it on untouched; it
//! checks nothing, because the TPM does, under a key the kernel holds itself.

use nonos_boot::handoff::types::AttestPolicy;
use nonos_boot::loader::file::load_file_from_esp;
use uefi::prelude::*;

/// Key x, y, then r, s.
const APPROVAL_LEN: usize = 128;

/// `policy` with the approval file attached, when there is one of the right
/// length. Absent or malformed, the kernel simply has no approval, and the
/// device secret stays sealed.
pub fn with_approval(st: &SystemTable<Boot>, mut policy: AttestPolicy) -> AttestPolicy {
    if let Ok(bytes) = load_file_from_esp(st, uefi::cstr16!("\\EFI\\nonos\\kernel.approval")) {
        if bytes.len() == APPROVAL_LEN {
            policy.approval.copy_from_slice(&bytes);
            policy.approval_present = 1;
        }
    }
    policy
}
