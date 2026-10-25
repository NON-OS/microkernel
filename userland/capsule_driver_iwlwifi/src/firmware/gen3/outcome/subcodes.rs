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

//! The codes a wait error and a start error add to a detail word.

use super::super::dev::WaitError;
use super::super::start::StartError;

pub(super) fn wait_code(w: WaitError) -> u32 {
    match w {
        WaitError::TimedOut => 1,
        WaitError::FirmwareError => 2,
        WaitError::TooLarge => 3,
        WaitError::Busy => 4,
    }
}

pub(super) fn start_code(e: StartError) -> u32 {
    match e {
        StartError::NotReady => 1,
        StartError::ClockNotReady => 2,
        StartError::NoNicAccess => 3,
    }
}
