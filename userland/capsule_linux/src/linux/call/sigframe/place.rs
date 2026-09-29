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

//! Where a signal frame lands on a thread's stack.

use super::layout::FRAME_SIZE;

/// The System V red zone below rsp.
const REDZONE: u64 = 128;

/// True when `rsp` is on the alternate stack `(base, size)`.
pub(super) fn on_alt(alt: Option<(u64, u64)>, rsp: u64) -> bool {
    alt.is_some_and(|(sp, size)| rsp > sp && rsp - sp <= size)
}

/// The frame's address as Linux's `get_sigframe` picks it, or `None` when
/// the frame does not fit.
pub(super) fn place(rsp: u64, alt: Option<(u64, u64)>, onstack: bool) -> Option<u64> {
    /*
     * Below the red zone, or at the top of the alternate stack for a handler
     * that asked for it when the thread is not already running there. Then
     * 16-aligned and down 8, so the handler sees rsp+8 aligned as a call
     * would leave it.
     */
    let top = match alt {
        Some((sp, size)) if onstack && !on_alt(alt, rsp) => sp.checked_add(size)?,
        _ => rsp.checked_sub(REDZONE)?,
    };
    let frame = (top.checked_sub(FRAME_SIZE as u64)? & !15u64).checked_sub(8)?;
    /*
     * A frame that would run off the bottom of the alternate stack, whether
     * the handler enters it or the thread is on it already, is not written
     * over whatever lies below it.
     */
    if let Some((sp, _)) = alt.filter(|_| onstack || on_alt(alt, rsp)) {
        if frame <= sp {
            return None;
        }
    }
    Some(frame)
}
