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

//! SAM-5 status codes.

pub const GOOD: u8 = 0x00;
pub const CHECK_CONDITION: u8 = 0x02;
pub const CONDITION_MET: u8 = 0x04;
pub const BUSY: u8 = 0x08;
pub const RESERVATION_CONFLICT: u8 = 0x18;
pub const TASK_SET_FULL: u8 = 0x28;
pub const ACA_ACTIVE: u8 = 0x30;
pub const TASK_ABORTED: u8 = 0x40;
