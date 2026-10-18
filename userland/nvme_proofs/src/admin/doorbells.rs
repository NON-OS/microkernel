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

//! The real admin doorbell offsets, which only the admin module sees.

use super::queue::{cq0_head, sq0_tail};

/// The admin submission tail and completion head doorbells for a stride.
pub fn admin_doorbells(stride: u8) -> (u32, u32) {
    (sq0_tail::sq0_tail(stride), cq0_head::cq0_head(stride))
}
