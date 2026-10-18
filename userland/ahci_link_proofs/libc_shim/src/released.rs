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

//! How a device goes back, read off the call log. The kernel's release reads
//! Bus Master back off before it frees the device's DMA grants; an unmap by
//! hand comes with no such stop, and the broker ORs Command bits until K6, so
//! the driver's own "off" write cannot stop the device first.

use crate::calls::{Call, Engines, Seen};
use crate::checks::bus_master;

/// The release is the last call, and no DMA grant went back by hand before
/// it, nor twice: the release frees them.
pub fn released_last(seen: &[Seen]) {
    let by_hand = |s: &Seen| matches!(s.call, Call::DmaUnmap | Call::StaleUnmap);
    assert!(!seen.iter().any(by_hand), "a grant went back by hand: {seen:?}");
    assert_eq!(seen.last().map(|s| s.call), Some(Call::Release), "{seen:?}");
}

/// A Command write turned Bus Master off after it last went on, ahead of
/// the release: the device as it read at that last on, and at the off.
pub fn bus_master_off(seen: &[Seen]) -> (Engines, Engines) {
    let on = seen.iter().rposition(|s| bus_master(s, true)).expect("Bus Master went on");
    let off = seen[on..].iter().position(|s| bus_master(s, false)).map(|at| on + at);
    let off = off.unwrap_or_else(|| panic!("Bus Master never written off: {seen:?}"));
    let release = seen.iter().position(|s| s.call == Call::Release);
    assert!(release.is_some_and(|r| off < r), "not off before the release: {seen:?}");
    (seen[on].device, seen[off].device)
}

/// As `bus_master_off`, with a reset seen to end in between (the model
/// counts only those that stop every engine) and the engines read stopped.
pub fn bus_master_off_after_stop(seen: &[Seen]) {
    let (was, is) = bus_master_off(seen);
    assert!(is.resets > was.resets && is.running == 0, "off before a stop: {seen:?}");
}
