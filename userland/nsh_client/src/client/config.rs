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

//! What the caller tells the client before it connects.

use crate::Policy;
use alloc::vec::Vec;

pub struct Config {
    /// The host name the user typed. The enrolment must name it.
    pub host: Vec<u8>,
    /// The caller's clock, for the enrolment's expiry.
    pub now_ms: u64,
    pub policy: Policy,
    /// The user's base grant, `nsh-grant/1`.
    pub grant: Vec<u8>,
}
