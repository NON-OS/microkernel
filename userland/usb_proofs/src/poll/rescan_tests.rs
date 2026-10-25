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

//! When the poll loop looks at the ports: every 64 ms while no report comes
//! in, first 64 ms after the last one.

use super::rescan::{Rescan, RESCAN_MS};

#[test]
fn the_ports_are_looked_at_every_64_ms_while_quiet_and_not_while_reports_come() {
    let mut r = Rescan::new(0);
    let quiet: Vec<u64> = (0..300).filter(|&t| r.rescan(t)).collect();
    assert_eq!(quiet, [64, 128, 192, 256]);
    let mut r = Rescan::new(0);
    let mut busy = Vec::new();
    for t in 0..1_000 {
        if t < 500 {
            r.heard(t);
        }
        if r.rescan(t) {
            busy.push(t);
        }
    }
    assert_eq!(busy[0], 499 + RESCAN_MS, "{busy:?}");
    assert!(busy.windows(2).all(|w| w[1] - w[0] == RESCAN_MS));
}
