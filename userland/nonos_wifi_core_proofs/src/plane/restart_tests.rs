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

//! The chip's restart (E-53) seen from the data plane. While the chip is
//! dead — it refuses to send and hears nothing — every transmit the stack
//! offers is refused and no new data reaches the AP, and no frame is
//! delivered up. E-53 proved no cleartext crosses the restart; this holds
//! the netif face shut across the dead window.

use super::drive::{rx, tx};
use crate::chip::restart_rig;

#[test]
fn the_plane_is_shut_while_the_chip_is_down() {
    // Baseline: the radio calls a clean join uses this many calls.
    let mut base = restart_rig::station(1_000_000); // never dead
    assert_eq!(restart_rig::connect(&mut base), 0);
    let joined_calls = base.radio().unwrap().calls;
    // Restart on the first radio call after the join completes.
    let mut st = restart_rig::station(joined_calls + 1);
    assert_eq!(restart_rig::connect(&mut st), 0);
    let got = st.radio().unwrap().air.ap.got.len();
    // Stay within the dead window (40 calls): every op here finds it dead.
    for _ in 0..16 {
        assert!(tx(&mut st, 7) < 0, "transmit refused while the chip is down");
        assert!(!rx(&mut st), "no frame delivered while the chip is down");
    }
    assert_eq!(st.radio().unwrap().air.ap.got.len(), got, "no new data reached the AP");
}
