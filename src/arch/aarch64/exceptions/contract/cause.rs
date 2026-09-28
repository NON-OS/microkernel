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

use crate::arch::aarch64::exceptions::frame::ExceptionFrame;
use crate::arch::aarch64::exceptions::syndrome::ExceptionClass;
use crate::arch::trap::contract::TrapCause;

use super::page_fault;

/// `OtherException` carries the architectural EC from ESR_EL1[31:26], not
/// the position of its `ExceptionClass` variant, so the number printed is
/// the one the Arm ARM tables list.
pub(super) fn project(frame: &ExceptionFrame) -> TrapCause {
    let raw = ((frame.esr >> 26) & 0x3F) as u8;
    let ec = ExceptionClass::from(raw);
    match ec {
        ExceptionClass::DataAbortLower | ExceptionClass::DataAbortSame => {
            TrapCause::PageFault(page_fault::decode_data(frame))
        }
        ExceptionClass::InstructionAbortLower | ExceptionClass::InstructionAbortSame => {
            TrapCause::PageFault(page_fault::decode_instruction(frame))
        }
        ExceptionClass::PcAlignment | ExceptionClass::SpAlignment => TrapCause::Alignment,
        ExceptionClass::IllegalState => TrapCause::InvalidOpcode,
        ExceptionClass::SError => TrapCause::MachineCheck,
        ExceptionClass::FpAccess
        | ExceptionClass::SveAccess
        | ExceptionClass::Fp32
        | ExceptionClass::Fp64 => TrapCause::DeviceNotAvailable,
        ExceptionClass::Pac | ExceptionClass::EretEretaa | ExceptionClass::BranchTarget => {
            TrapCause::OtherException(raw)
        }
        ExceptionClass::Unknown
        | ExceptionClass::WfeWfi
        | ExceptionClass::Cp15Mcr
        | ExceptionClass::Cp15Mcrr
        | ExceptionClass::Cp14Mcr
        | ExceptionClass::Cp14Ldc
        | ExceptionClass::Cp14Mrrc
        | ExceptionClass::SysReg
        | ExceptionClass::Hvc64
        | ExceptionClass::Smc64
        | ExceptionClass::Svc32
        | ExceptionClass::Svc64
        | ExceptionClass::BreakpointLower
        | ExceptionClass::BreakpointSame
        | ExceptionClass::SoftwareStepLower
        | ExceptionClass::SoftwareStepSame
        | ExceptionClass::WatchpointLower
        | ExceptionClass::WatchpointSame
        | ExceptionClass::Bkpt32
        | ExceptionClass::Brk64 => TrapCause::OtherException(raw),
    }
}
