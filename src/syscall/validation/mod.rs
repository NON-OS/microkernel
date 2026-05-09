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

//! Userspace pointer validation for the syscall surface. The single
//! authoritative usercopy implementation lives at `crate::usercopy`;
//! syscall handlers must call `crate::usercopy::{validate_user_read,
//! validate_user_write, copy_from_user, copy_to_user, read_user_string}`
//! directly. This module deliberately re-exports nothing — a second
//! validator would duplicate page-walk logic and drift over time.
