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

use super::error::WireError;
use super::status::lift;

#[test]
fn every_errno_the_wire_names_lifts_to_its_own_error() {
    let named = [
        (-5, WireError::DeviceFailure),
        (-6, WireError::OutOfRange),
        (-11, WireError::NotReady),
        (-13, WireError::AccessDenied),
        (-19, WireError::NoDevice),
        (-22, WireError::InvalidArgument),
        (-30, WireError::ReadOnly),
        (-90, WireError::OversizedRequest),
        (-110, WireError::TimedOut),
        (-123, WireError::NoMedium),
    ];
    for (status, want) in named {
        assert_eq!(lift(status), want, "{status}");
    }
}

#[test]
fn an_unknown_status_is_a_failure_never_success() {
    for status in [-1, -4, -999, i32::MIN, 7, i32::MAX] {
        assert_eq!(lift(status), WireError::DeviceFailure, "{status}");
    }
}
