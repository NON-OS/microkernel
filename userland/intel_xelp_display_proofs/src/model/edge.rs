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
//! A limit test: the timing at the limit is taken, one step past is refused.

use super::mode::timing;
use nonos_edid::Detailed;
use nonos_intel_xelp_display::timing::check::{check, Sink};

/// 640 active, 160 blank (16 front, 96 sync); 480 lines, 45 (10, 2).
pub fn base() -> Detailed {
    timing(25_175, [640, 160, 16, 96], [480, 45, 10, 2])
}

/// `f(t, false)` puts `t` at a limit and `f(t, true)` one step past it.
pub fn edge(sink: Sink, f: impl Fn(&mut Detailed, bool)) {
    let (mut at, mut past) = (base(), base());
    f(&mut at, false);
    f(&mut past, true);
    assert!(check(&at, sink).is_ok(), "at the limit refused: {at:?}");
    assert!(check(&past, sink).is_err(), "past the limit taken: {past:?}");
}
