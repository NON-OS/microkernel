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

//! MkAcpiPci's narrowing for bridges, the other route to L58's hazard: AML
//! never rewrites a bridge's bus numbers, windows or secondary status, and
//! never turns its bus mastering on or off. Reads are not narrowed.

use super::refusal::Refusal;

const COMMAND: u16 = 0x04;
const BUS_MASTER: u32 = 1 << 2;
/// Primary, secondary and subordinate bus, the I/O, memory and prefetchable
/// windows and their upper halves, and secondary status (header type 1;
/// type 2 keeps its bus numbers and windows in the same span).
pub const ROUTING: (u16, u16) = (0x18, 0x30);

/// Whether a write of `width` bytes of `value` at `at` may go to a function
/// whose Header Type byte is `header` and whose Command register reads
/// `command`. A Command write that keeps bus mastering as it is passes,
/// since AML writes a field's whole access unit back.
pub fn bridge_write(
    header: u8,
    at: u16,
    width: u8,
    value: u32,
    command: u16,
) -> Result<(), Refusal> {
    if !matches!(header & 0x7F, 1 | 2) {
        return Ok(());
    }
    let end = at.saturating_add(u16::from(width));
    if at < ROUTING.1 && ROUTING.0 < end {
        return Err(Refusal::Bridge);
    }
    if at <= COMMAND && COMMAND < end {
        let asked = (value >> (u32::from(COMMAND - at) * 8)) & BUS_MASTER;
        if asked != u32::from(command) & BUS_MASTER {
            return Err(Refusal::Bridge);
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tell {
    /// A (function, offset) not seen before: one line, remembered.
    New,
    /// The bound reached: one last line saying the rest go untold.
    Last,
    Quiet,
}

/// Whether a refused bridge write is logged: once per (function, offset)
/// for the first `bound` of them, then once more to say so, then never, so
/// AML looping over offsets cannot fill the log (F-95).
pub fn tell(known: bool, remembered: usize, bound: usize) -> Tell {
    match (known, remembered.cmp(&bound)) {
        (true, _) | (false, core::cmp::Ordering::Greater) => Tell::Quiet,
        (false, core::cmp::Ordering::Less) => Tell::New,
        (false, core::cmp::Ordering::Equal) => Tell::Last,
    }
}
