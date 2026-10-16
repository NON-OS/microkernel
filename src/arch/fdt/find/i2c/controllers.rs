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

//! First pass: every enabled `snps,designware-i2c` node whose registers sit
//! at a CPU address, with its properties as written.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;
use crate::arch::fdt::walker::Event;

use super::find::MAX_DEPTH;
use super::level::Level;
use super::node::DwI2c;
use super::props::{controller, note};

/// The count found, which may pass `out.len()`; only the first are kept.
/// Nodes deeper than `MAX_DEPTH` are passed over.
pub(super) fn controllers(fdt: &Fdt, out: &mut [DwI2c]) -> Result<usize, FdtError> {
    let mut walker = fdt.walker();
    let mut stack = [Level::root(); MAX_DEPTH];
    let mut open = 0usize;
    let mut found = 0;
    while let Some(event) = walker.next()? {
        match event {
            Event::BeginNode { .. } if open > 0 && open < MAX_DEPTH => {
                let p = &stack[open - 1];
                stack[open] = Level::child_of(p.irq_parent_in_force(), p.passes_down(open == 1));
                open += 1;
            }
            Event::BeginNode { .. } => open += 1,
            Event::EndNode => {
                let Some(at) = open.checked_sub(1) else { break };
                open = at;
                if let Some(c) = stack.get(at).and_then(controller) {
                    if let Some(slot) = out.get_mut(found) {
                        *slot = c;
                    }
                    found += 1;
                }
            }
            Event::Property(p) if (1..=MAX_DEPTH).contains(&open) => {
                let parent = stack[open.saturating_sub(2)].cells;
                note(&mut stack[open - 1], &p, parent)?;
            }
            Event::Property(_) => {}
        }
    }
    Ok(found)
}
