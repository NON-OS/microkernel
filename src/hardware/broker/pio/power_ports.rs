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

//! A PIO grant never covers the FADT's power registers (PM1a and PM1b
//! control, sleep control, reset): the kernel enters soft-off and resets
//! through them itself, after the wipe. A driver's BAR over one is refused
//! as a reserved port.

use super::window::Refused;
use crate::hardware::broker::acpi::{kernel::power_regs, rule::SPACE_IO};

/// The window `w` gives (base, count), or `Reserved` when a power register
/// is in it.
pub(super) fn clear(w: Result<(u16, u16), Refused>) -> Result<(u16, u16), Refused> {
    let (base, count) = w?;
    let (lo, hi) = (u64::from(base), u64::from(base) + u64::from(count));
    if power_regs::touches(SPACE_IO, lo, hi) {
        return Err(Refused::Reserved);
    }
    Ok((base, count))
}
