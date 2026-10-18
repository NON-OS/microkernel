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

use super::describe;
use crate::arch::fdt::find::idle::Refusal;
use crate::arch::fdt::Fdt;
use crate::sys::serial::Line;

/// The tree's idle states, kept for the idle path. Nothing here refuses the
/// tree: a damaged description costs only the states, and says so.
pub(super) fn populate(fdt: &Fdt) -> Result<(), &'static [u8]> {
    // The frame that keeps time for cores whose own timer stops in one.
    if let Err(e) = crate::arch::aarch64::timer::broadcast::describe(fdt) {
        let mut line = Line::new();
        line.str(b"[IDLE] memory-mapped timer not read: ").str(describe(e));
        line.str(b"; no state that stops the timer").end();
    }
    let reason: &[u8] = match crate::arch::aarch64::idle_states::describe(fdt) {
        Ok(_) => return Ok(()),
        Err(Refusal::Tree(e)) => describe(e),
        Err(Refusal::TooManyStates) => b"more idle states than kept",
        Err(Refusal::TooManyForCpu) => b"a CPU names more idle states than kept",
    };
    Line::new().str(b"[IDLE] states not read: ").str(reason).str(b"; WFI only").end();
    Ok(())
}
