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
//! The vertical limits, and what is refused outright.

use crate::model::edge::edge;
use crate::model::mode::cta_1080p;
use nonos_intel_xelp_display::timing::check::{check, Sink};

#[test]
fn vertical_limits() {
    let dp = Sink::DisplayPort;
    edge(dp, |t, p| t.v_active = if p { 0 } else { 1 });
    // Blank at least 5 lines (p. 1378); sync inside it (p. 1393).
    edge(dp, |t, p| (t.v_blank, t.v_front, t.v_sync) = (if p { 4 } else { 5 }, 1, 1));
    edge(dp, |t, p| t.v_front = if p { 0 } else { 1 });
    edge(dp, |t, p| t.v_sync = if p { 0 } else { 1 });
    edge(dp, |t, p| t.v_sync = if p { 35 } else { 34 });
    // Source height at most 4320 (p. 705).
    edge(dp, |t, p| t.v_active = if p { 4321 } else { 4320 });
    // Total within 13 bits (p. 1396): 4320 + 3872 = 8192.
    edge(dp, |t, p| (t.v_active, t.v_blank) = (4320, if p { 3873 } else { 3872 }));
}

#[test]
fn interlaced_and_clockless_refused() {
    let mut t = cta_1080p();
    t.interlaced = true;
    assert!(check(&t, Sink::DisplayPort).is_err());
    let mut t = cta_1080p();
    t.pixel_khz = 0;
    assert!(check(&t, Sink::DisplayPort).is_err());
}
