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
    match cause {
        TrapCause::PageFault(info) => page_fault(info),
        TrapCause::ProtectionFault { error_code }
        | TrapCause::StackSegment { error_code }
        | TrapCause::SegmentNotPresent { error_code }
        | TrapCause::InvalidTss { error_code }
        | TrapCause::ControlProtection { error_code }
        | TrapCause::DoubleFault { error_code } => {
            Line::new().str(b"  stval=").hex(*error_code).end_fatal();
        }
        TrapCause::OtherException(code) => {
            Line::new().str(b"  scause-code=").hex(*code as u64).end_fatal();
        }
        _ => {}
    }
}

fn page_fault(info: &PageFaultInfo) {
    let mut l = Line::new();
    l.str(b"  stval=").hex(info.fault_address).str(access_label(info.access).as_bytes());
    l.str(if info.user { b" U-mode" } else { b" S-mode" }).end_fatal();
}

fn access_label(access: FaultAccess) -> &'static str {
    match access {
        FaultAccess::Read => " load",
        FaultAccess::Write => " store",
        FaultAccess::InstructionFetch => " ifetch",
    }
}
