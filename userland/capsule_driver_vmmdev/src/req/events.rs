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

//! The pending events, read and cleared in one request; the host lowers
//! its pending flag with it.

use super::page::Page;
use crate::constants::req::ACKNOWLEDGE_EVENTS;

pub fn acknowledge(p: &mut Page) -> Result<u32, i32> {
    p.perform(ACKNOWLEDGE_EVENTS, &[0]).map(|b| b[0])
}
