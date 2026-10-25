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
use crate::syscall::SyscallResult;
use crate::usercopy::write_user_value;

const ENOTSUP: i32 = 95;
const EFAULT: i32 = 14;
const EINVAL: i32 = 22;

pub(super) fn matches(nr: SyscallNumber) -> bool {
    matches!(nr, SyscallNumber::GraphicsDisplayDimensions)
}

pub(super) fn handle(
    nr: SyscallNumber,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    _a5: u64,
) -> SyscallResult {
    match nr {
        SyscallNumber::GraphicsDisplayDimensions => handle_display_dimensions(a0, a1, a2, a3, a4),
        _ => super::super::util::errno(ENOTSUP),
    }
}

// `out_mm`, when not 0, receives the panel's physical size from its EDID,
// width in millimetres in the low 16 bits and height in the high 16, or 0
// when the firmware gave none. `out_why`, when not 0, receives why there is
// no framebuffer with ENOTSUP (boot_refusal: FbRefusal::why_word), and 0 with
// one. Callers that pass 0 for either see the old call.
fn handle_display_dimensions(
    display: u64,
    out_w: u64,
    out_h: u64,
    out_mm: u64,
    out_why: u64,
) -> SyscallResult {
    use super::super::util::errno;
    if display != 0 || out_w == 0 || out_h == 0 {
        return errno(EINVAL);
    }
    let Some(fb) = crate::kernel_core::init::framebuffer::framebuffer_state() else {
        let why = crate::kernel_core::init::framebuffer::boot_refusal();
        let told = out_why == 0 || write_user_value(out_why, &why).is_ok();
        return errno(if told { ENOTSUP } else { EFAULT });
    };
    let mm = fb.physical_mm.map_or(0u32, |(w, h)| w | (h << 16));
    // out_w and out_h are never 0 here; an optional pointer left 0 is skipped.
    let writes = [(out_w, fb.width), (out_h, fb.height), (out_mm, mm), (out_why, 0)];
    if writes.iter().any(|&(at, v)| at != 0 && write_user_value(at, &v).is_err()) {
        return errno(EFAULT);
    }
    SyscallResult::success_audited(0)
}
