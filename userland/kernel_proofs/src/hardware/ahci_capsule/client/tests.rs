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

use super::super::error::DriverAhciError as E;
use super::status_map::lift;

#[test]
fn no_disk_on_the_port_is_a_disk_that_is_not_there() {
    assert_eq!(lift(-19), E::Dead, "E_NODEV");
}

#[test]
fn the_other_statuses_keep_their_meaning() {
    assert_eq!(lift(-22), E::InvalidArgument);
    assert_eq!(lift(-5), E::DeviceFailure);
    assert_eq!(lift(-6), E::OutOfRange);
    assert_eq!(lift(-90), E::OversizedRequest);
    assert_eq!(lift(-1), E::DeviceFailure, "an undocumented status");
}
