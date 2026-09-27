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

use crate::arch::trap::contract::cause::{FaultAccess, PageFaultInfo, TrapCause};
use crate::sys::serial::Line;

pub(super) fn report(cause: &TrapCause) {
    let mut line = Line::new();
    line.str(b"[TRAP] ");
    match cause {
        TrapCause::PageFault(info) => page_fault(&mut line, info),
        TrapCause::ProtectionFault { error_code }
        | TrapCause::StackSegment { error_code }
        | TrapCause::SegmentNotPresent { error_code }
        | TrapCause::InvalidTss { error_code }
        | TrapCause::ControlProtection { error_code }
        | TrapCause::DoubleFault { error_code } => {
            line.str(b"esr=").hex(*error_code);
        }
        TrapCause::OtherException(ec) => {
            line.str(b"ec=").hex(u64::from(*ec));
        }
        _ => return,
    }
    line.end_fatal();
}

fn page_fault(line: &mut Line, info: &PageFaultInfo) {
    line.str(b"far=")
        .hex(info.fault_address)
        .str(access_label(info.access))
        .str(if info.present { b" present" } else { b" not-present" })
        .str(if info.user { b" EL0" } else { b" EL1" });
}

fn access_label(access: FaultAccess) -> &'static [u8] {
    match access {
        FaultAccess::Read => b" read",
        FaultAccess::Write => b" write",
        FaultAccess::InstructionFetch => b" ifetch",
    }
}
