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

//! Namespace modifiers and named objects, sections 20.2.5.1 and 20.2.5.2.

use super::Op;

impl Op {
    pub const ALIAS: Op = Op(0x06);
    pub const NAME: Op = Op(0x08);
    pub const SCOPE: Op = Op(0x10);
    pub const METHOD: Op = Op(0x14);
    pub const EXTERNAL: Op = Op(0x15);
    pub const CREATE_DWORD_FIELD: Op = Op(0x8A);
    pub const CREATE_WORD_FIELD: Op = Op(0x8B);
    pub const CREATE_BYTE_FIELD: Op = Op(0x8C);
    pub const CREATE_BIT_FIELD: Op = Op(0x8D);
    pub const CREATE_QWORD_FIELD: Op = Op(0x8F);
    pub const MUTEX: Op = Op::ext(0x01);
    pub const EVENT: Op = Op::ext(0x02);
    pub const CREATE_FIELD: Op = Op::ext(0x13);
    pub const OP_REGION: Op = Op::ext(0x80);
    pub const FIELD: Op = Op::ext(0x81);
    pub const DEVICE: Op = Op::ext(0x82);
    /// Deprecated since ACPI 6.4, still parsed: older firmware declares its
    /// processors with it.
    pub const PROCESSOR: Op = Op::ext(0x83);
    pub const POWER_RES: Op = Op::ext(0x84);
    pub const THERMAL_ZONE: Op = Op::ext(0x85);
    pub const INDEX_FIELD: Op = Op::ext(0x86);
    pub const BANK_FIELD: Op = Op::ext(0x87);
    pub const DATA_REGION: Op = Op::ext(0x88);
}
