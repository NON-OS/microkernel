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
//! Length-prefixed fields, read and written in one place: a u32 LE length,
//! then the bytes. A length the request does not hold, or one too long to
//! state, is refused.

mod pair;
mod take;
mod write;

pub(crate) use pair::parse_pair;
pub(crate) use take::{read_lp_str, take_lp};
pub(crate) use write::{write_count, write_lp};
