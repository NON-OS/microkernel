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

//! The input router's request header and the ops the shell sends.

pub(super) const MAGIC: u32 = 0x4E49_5253;
pub(super) const VERSION: u16 = 1;
pub(super) const HDR_LEN: usize = 20;
pub(super) const STATUS_LEN: usize = 4;
pub(super) const BODY_LEN: usize = 8;
pub(super) const OP_SUBSCRIBE: u16 = 0x0002;
pub(super) const OP_GRAB_REQUEST: u16 = 0x0003;
pub(super) const OP_GRAB_RELEASE: u16 = 0x0004;
pub(super) const REPLY_TIMEOUT_MS: u64 = 250;
