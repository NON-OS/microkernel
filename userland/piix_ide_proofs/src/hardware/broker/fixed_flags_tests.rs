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

//! A compatibility channel's window carries the fixed-window flag and none
//! of a BAR's own: an ISA port range is no prefetchable or 64-bit window.

use super::device::{BAR_FLAG_FIXED, BAR_FLAG_MEM64, BAR_FLAG_PREFETCH};
use super::{fill, Bar};

#[test]
fn a_fixed_window_is_flagged_fixed_and_nothing_else() {
    let mut bars = [Bar { base: 0, size: 0, kind: 0, flags: 0, aux: 0, _pad: [0; 2] }; 6];
    let mut span = 0;
    fill(1, 1, 0x80, &mut bars, &mut span);
    for b in &bars[..4] {
        assert_eq!((b.flags, b.flags & (BAR_FLAG_PREFETCH | BAR_FLAG_MEM64)), (BAR_FLAG_FIXED, 0));
    }
}
