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

//! The orders a bring-up is held to, read off the call log.

use crate::calls::{Call, Seen};
use crate::released::released_last;
use crate::MK_PCI_CMD_BUS_MASTER;

/// A Command write the broker took, with Bus Master on in it, or off.
pub(crate) fn bus_master(s: &Seen, on: bool) -> bool {
    let word = matches!(s.call, Call::Command(w) if (w & MK_PCI_CMD_BUS_MASTER != 0) == on);
    word && !s.refused
}

/// The first Command write that turned Bus Master on, if any: one the
/// broker refused turned nothing on.
pub fn first_bus_master(seen: &[Seen]) -> Option<Seen> {
    seen.iter().copied().find(|s| bus_master(s, true))
}

/// Bus Master went on, and not before the device had finished a reset and
/// read every engine stopped; no DMA grant was made before that either.
pub fn bus_master_after_reset(seen: &[Seen]) {
    let on = first_bus_master(seen).unwrap_or_else(|| panic!("Bus Master never went on: {seen:?}"));
    let stopped = |s: &Seen| s.device.resets > 0 && s.device.running == 0;
    assert!(stopped(&on), "Bus Master went on before the engines were stopped: {on:?} in {seen:?}");
    for s in seen.iter().filter(|s| s.call == Call::DmaMap) {
        assert!(stopped(s), "a DMA grant was made before the reset was seen: {s:?} in {seen:?}");
    }
}

/// A device refused: Bus Master never asked for, and given back as
/// `released_last` has it.
pub fn refused_without_bus_master(seen: &[Seen]) {
    if let Some(on) = first_bus_master(seen) {
        panic!("a refused device was given Bus Master: {on:?} in {seen:?}");
    }
    released_last(seen);
}
