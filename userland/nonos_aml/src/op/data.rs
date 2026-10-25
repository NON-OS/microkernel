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

//! Data objects, section 20.2.3, and the local, argument and debug objects of
//! section 20.2.6.

use super::Op;

impl Op {
    pub const ZERO: Op = Op(0x00);
    pub const ONE: Op = Op(0x01);
    pub const BYTE_CONST: Op = Op(0x0A);
    pub const WORD_CONST: Op = Op(0x0B);
    pub const DWORD_CONST: Op = Op(0x0C);
    pub const STRING: Op = Op(0x0D);
    pub const QWORD_CONST: Op = Op(0x0E);
    pub const BUFFER: Op = Op(0x11);
    pub const PACKAGE: Op = Op(0x12);
    pub const VAR_PACKAGE: Op = Op(0x13);
    pub const ONES: Op = Op(0xFF);
    pub const REVISION: Op = Op::ext(0x30);
    pub const LOCAL0: Op = Op(0x60);
    pub const LOCAL7: Op = Op(0x67);
    pub const ARG0: Op = Op(0x68);
    pub const ARG6: Op = Op(0x6E);
    pub const DEBUG: Op = Op::ext(0x31);
}
