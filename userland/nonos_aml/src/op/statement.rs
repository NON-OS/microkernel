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

//! Statements (Type 1 opcodes), section 20.2.5.3.

use super::Op;

impl Op {
    pub const NOTIFY: Op = Op(0x86);
    pub const CONTINUE: Op = Op(0x9F);
    pub const IF: Op = Op(0xA0);
    pub const ELSE: Op = Op(0xA1);
    pub const WHILE: Op = Op(0xA2);
    pub const NOOP: Op = Op(0xA3);
    pub const RETURN: Op = Op(0xA4);
    pub const BREAK: Op = Op(0xA5);
    pub const BREAK_POINT: Op = Op(0xCC);
    pub const STALL: Op = Op::ext(0x21);
    pub const SLEEP: Op = Op::ext(0x22);
    pub const SIGNAL: Op = Op::ext(0x24);
    pub const RESET: Op = Op::ext(0x26);
    pub const RELEASE: Op = Op::ext(0x27);
    pub const UNLOAD: Op = Op::ext(0x2A);
    pub const FATAL: Op = Op::ext(0x32);
}
