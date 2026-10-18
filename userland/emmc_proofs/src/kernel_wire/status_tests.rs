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

//! The statuses driver.sdhci0 answers, as the kernel client reads them.

use super::error::WireError;
use super::status::lift;
use crate::emmc::error::{CARD_ERROR_BASE, HOST_ERROR_BASE};
use crate::protocol as c;

#[test]
fn the_statuses_the_capsule_answers_keep_their_meaning() {
    assert_eq!(lift(c::E_NXIO), WireError::OutOfRange);
    assert_eq!(lift(c::E_ACCES), WireError::AccessDenied);
    assert_eq!(lift(c::E_INVAL), WireError::InvalidArgument);
    assert_eq!(lift(c::E_MSGSIZE), WireError::OversizedRequest);
    assert_eq!(lift(-110), WireError::TimedOut);
    // A host or card status carrying its own detail is a failed I/O.
    assert_eq!(lift(-(HOST_ERROR_BASE | 0x12_0010)), WireError::DeviceFailure);
    assert_eq!(lift(-(CARD_ERROR_BASE | 0x19_001f)), WireError::DeviceFailure);
}
