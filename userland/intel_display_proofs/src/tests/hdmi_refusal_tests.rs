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

//! HDMI outputs the driver refuses to clock.

use super::hdmi_tests::lit_port;
use crate::engine::ddi::buf_trans;
use crate::engine::error::Error;
use crate::engine::hdmi::bring_up;
use crate::model::Ticks;

#[test]
fn a_clock_past_the_sinks_tmds_limit_or_an_unlit_port_is_refused_untouched() {
    let mut m = lit_port();
    assert_eq!(
        bring_up(&mut m, &Ticks::new(), 3, 2, 297_000, 0),
        Err(Error::Argument),
        "165 MHz by default"
    );
    assert_eq!(
        bring_up(&mut m, &Ticks::new(), 3, 0, 148_500, 0),
        Err(Error::Argument),
        "never DPLL 0"
    );
    m.set(buf_trans(3, 0), 0);
    assert_eq!(bring_up(&mut m, &Ticks::new(), 3, 2, 148_500, 0), Err(Error::Layout));
    assert!(m.writes.is_empty());
}
