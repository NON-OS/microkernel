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

//! The data objects that carry operands, section 20.2.3. The constants and
//! strings have fixed encodings and are read without a row.

use super::{Arg::*, Class::Data, Group};

pub const DATA: &[Group] = &[
    // DefBuffer: BufferSize, then the initializer.
    Group { class: Data, pkg: true, args: &[Term, Bytes], ops: &[0x11] },
    // DefPackage: NumElements.
    Group { class: Data, pkg: true, args: &[U8, Elements], ops: &[0x12] },
    // DefVarPackage: VarNumElements.
    Group { class: Data, pkg: true, args: &[Term, Elements], ops: &[0x13] },
];
