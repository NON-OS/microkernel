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

//! A SYNC heard while Active is a peer that started over (Vol 4, Part D,
//! "Link Establishment"): this side starts over too, and answers it.

use super::h5_link_tests::{active_with_window_one, SYNC_RESPONSE};

#[test]
fn a_sync_while_active_starts_this_side_over_and_is_answered() {
    let mut h5 = active_with_window_one();
    h5.feed(&[0xc0, 0x00, 0x2f, 0x00, 0xd0, 0x01, 0x7e, 0xc0], 3);
    assert!(!h5.active());
    assert_eq!(h5.restarts, 1);
    assert_eq!(h5.poll_out(3, 250), Some(SYNC_RESPONSE.to_vec()));
}
