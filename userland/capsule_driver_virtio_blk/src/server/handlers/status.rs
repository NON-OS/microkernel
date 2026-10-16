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

use crate::io::BlkError;
use crate::protocol::{E_INVAL, E_IO, E_TIMEDOUT};

/// The wire status a request's outcome is answered with. A timeout is
/// E_TIMEDOUT, so a client can tell a device that stopped answering from
/// one that refused the sectors.
pub fn status_of(outcome: Result<(), BlkError>) -> i32 {
    match outcome {
        Ok(()) => 0,
        Err(BlkError::Refused) => E_INVAL,
        Err(BlkError::Timeout) => E_TIMEDOUT,
        Err(BlkError::Io) => E_IO,
    }
}
