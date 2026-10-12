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

//! What each call asks of the caller: PDUs to send on the ATT fixed
//! channel (CID 0x0004, Vol 3, Part A, 2.1), input events to post, and
//! how discovery ended.

use crate::error::HogpError;
use crate::report::Input;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Notifications are on; which boot reports were found.
    Ready {
        keyboard: bool,
        mouse: bool,
    },
    Failed(HogpError),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Step {
    pub send: Vec<Vec<u8>>,
    pub input: Vec<Input>,
    pub outcome: Option<Outcome>,
}

impl Step {
    pub fn send(pdu: Vec<u8>) -> Self {
        Self { send: alloc::vec![pdu], ..Self::default() }
    }

    pub fn input(input: Vec<Input>) -> Self {
        Self { input, ..Self::default() }
    }
}
