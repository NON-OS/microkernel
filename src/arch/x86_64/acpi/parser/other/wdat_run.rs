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

//! Running one WDAT action, its instructions in table order, through a
//! bus the caller gives. A read takes the register shifted down by its bit
//! offset and masked: read value answers whether it equals the value (1 or
//! 0), read countdown answers it. A write puts the value (write countdown:
//! the caller's count) masked and shifted up, keeping the other bits when
//! told to preserve. An action with a write runs only with `writes` set.
//! Recalled from Microsoft's WDAT description: unverified.

use super::wdat_decode::{Instruction, Kind, Reg};
use alloc::vec::Vec;
pub trait WdatBus {
    fn read(&mut self, r: &Reg) -> Option<u64>;
    fn write(&mut self, r: &Reg, v: u64) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunError {
    NoAction,
    /// The action writes, and writes are off.
    Switched,
    Bus,
}

/// The last read's answer; 0 for an action that only writes.
pub fn run(
    entries: &[Instruction],
    action: u8,
    count: u32,
    writes: bool,
    bus: &mut impl WdatBus,
) -> Result<u64, RunError> {
    let steps: Vec<&Instruction> = entries.iter().filter(|i| i.action == action).collect();
    let writing = |i: &&Instruction| matches!(i.kind, Kind::WriteValue | Kind::WriteCountdown);
    match (steps.is_empty(), writes || !steps.iter().any(writing)) {
        (true, _) => return Err(RunError::NoAction),
        (_, false) => return Err(RunError::Switched),
        _ => {}
    }
    let mut answer = 0;
    for i in steps {
        let (shift, mask) = (u32::from(i.reg.bit_offset), u64::from(i.mask));
        let new = match i.kind {
            Kind::ReadValue | Kind::ReadCountdown => {
                let v = bus.read(&i.reg).ok_or(RunError::Bus)? >> shift & mask;
                let equal = u64::from(v == u64::from(i.value));
                answer = if i.kind == Kind::ReadValue { equal } else { v };
                continue;
            }
            Kind::WriteValue => i.value,
            Kind::WriteCountdown => count,
        };
        let mut v = (u64::from(new) & mask) << shift;
        if i.preserve {
            v |= bus.read(&i.reg).ok_or(RunError::Bus)? & !(mask << shift);
        }
        bus.write(&i.reg, v).then_some(()).ok_or(RunError::Bus)?;
    }
    Ok(answer)
}
