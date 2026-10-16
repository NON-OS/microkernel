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

//! How often MkAcpiPci's refused bridge writes reach the log (F-95).

use super::rule::{tell, Tell};

/// F-95: a refused bridge write is told once per (function, offset) for
/// the first `bound`, then once to say the rest go untold, then never.
#[test]
fn refusals_stop_being_told_past_the_bound() {
    let (mut remembered, mut lines) = (0usize, 0usize);
    for offset in 0..1000u16 {
        match tell(false, remembered, 3) {
            Tell::New | Tell::Last => (remembered, lines) = (remembered + 1, lines + 1),
            Tell::Quiet => {}
        }
        let _ = offset;
    }
    assert_eq!(lines, 4, "three offsets, then one line saying so");
    assert_eq!(tell(true, 0, 3), Tell::Quiet, "a known offset is never told again");
    assert_eq!(tell(false, 3, 3), Tell::Last);
    assert_eq!(tell(false, 4, 3), Tell::Quiet);
}
