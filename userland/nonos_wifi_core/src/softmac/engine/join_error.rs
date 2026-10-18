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

//! The connect op's answer to a join that never reached the AP.

use crate::softmac::control::result::{CODE_NOT_FOUND, CODE_NO_ADDRESS, CODE_NO_ENTROPY};
use crate::softmac::control::ConnectResult;
use crate::softmac::join::JoinError;

pub(super) fn join_error(e: JoinError) -> ConnectResult {
    match e {
        JoinError::NotFound(beacons) => {
            let mut res = ConnectResult::code(CODE_NOT_FOUND);
            res.counters.recv = beacons;
            res
        }
        JoinError::NoEntropy => ConnectResult::code(CODE_NO_ENTROPY),
        JoinError::NoAddress => ConnectResult::code(CODE_NO_ADDRESS),
    }
}
