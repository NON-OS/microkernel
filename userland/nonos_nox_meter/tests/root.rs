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

//! The meter's root label, pinned byte for byte against C2c's prefix.

use nonos_nox_meter::root::{root_label, BOUND_PREFIX, ROOT_LABEL_LEN};

#[test]
fn the_label_is_the_bound_prefix_then_the_meters_name() {
    assert_eq!(BOUND_PREFIX, 0x01);
    let label = root_label();
    assert_eq!(ROOT_LABEL_LEN, 20);
    assert_eq!(label[0], 0x01);
    assert_eq!(&label[1..], b"nonos.meter.root.v1");
}
