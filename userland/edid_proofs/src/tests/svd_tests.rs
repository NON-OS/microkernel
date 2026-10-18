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

//! Short video descriptors (CTA-861-H section 7.5.1).

use crate::cta::video::svd;

#[test]
fn svd_codes_split_into_vic_and_native_as_the_revisions_define() {
    assert_eq!(svd(16), Some((16, false)));
    assert_eq!(svd(127), Some((127, false)));
    assert_eq!(svd(128 + 16), Some((16, true)));
    assert_eq!(svd(192), Some((64, true)));
    assert_eq!(svd(193), Some((193, false)));
    assert_eq!(svd(253), Some((253, false)));
    for r in [0, 128, 254, 255] {
        assert_eq!(svd(r), None, "{r} is reserved");
    }
}
