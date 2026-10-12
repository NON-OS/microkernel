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

//! A counter rate learned after the anchor, from a reference only available
//! later in boot.

use super::boot::COUNTER_HZ;
use core::sync::atomic::Ordering;

/// Fill in the counter rate when the anchor could not measure it, from a
/// reference only available later in boot (the ACPI PM timer, once the FADT
/// is parsed). A rate already latched is never replaced: elapsed times
/// handed out so far were measured against it.
pub(crate) fn set_counter_hz_if_unknown(hz: u64) -> bool {
    COUNTER_HZ.compare_exchange(0, hz, Ordering::AcqRel, Ordering::Relaxed).is_ok()
}

/// Whether the anchor latched a real rate.
pub(crate) fn counter_hz_known() -> bool {
    COUNTER_HZ.load(Ordering::Relaxed) != 0
}
