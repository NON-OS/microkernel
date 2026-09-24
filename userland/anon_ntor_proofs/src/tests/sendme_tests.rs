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

//! The SENDME body, against sendme_cell_encode.

use crate::stream::sendme::body;

#[test]
fn the_body_is_version_one_then_the_digest() {
    let digest = [0xabu8; 20];
    let built = body(&digest);
    assert_eq!(built.len(), 23);
    assert_eq!(built[0], 1);
    assert_eq!(&built[1..3], &[0, 20]);
    assert_eq!(&built[3..], &digest[..]);
}
