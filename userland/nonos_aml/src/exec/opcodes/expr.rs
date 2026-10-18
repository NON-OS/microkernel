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

//! Expressions that compute, convert or reach (section 20.2.5.4).

use super::{run, OpRow};

const INT: &str = "integer arithmetic at the table's width";
const TEXT: &str = "a conversion, its result counted against the allocation budget";
const REF: &str = "a reference, never past the end of its object";

pub const ROWS: &[OpRow] = &[
    run(0x70, "Store", "a value written to a target, converted as the target needs"),
    run(0x71, "RefOf", REF),
    run(0x72, "Add", INT),
    run(0x73, "Concatenate", TEXT),
    run(0x74, "Subtract", INT),
    run(0x75, "Increment", INT),
    run(0x76, "Decrement", INT),
    run(0x77, "Multiply", INT),
    run(0x78, "Divide", "integer division; by zero, a fault"),
    run(0x79, "ShiftLeft", INT),
    run(0x7A, "ShiftRight", INT),
    run(0x7B, "And", INT),
    run(0x7C, "NAnd", INT),
    run(0x7D, "Or", INT),
    run(0x7E, "NOr", INT),
    run(0x7F, "XOr", INT),
    run(0x80, "Not", INT),
    run(0x81, "FindSetLeftBit", INT),
    run(0x82, "FindSetRightBit", INT),
    run(0x83, "DerefOf", REF),
    run(0x84, "ConcatenateResTemplate", TEXT),
    run(0x85, "Mod", "integer remainder; by zero, a fault"),
    run(0x87, "SizeOf", REF),
    run(0x88, "Index", REF),
    run(0x89, "Match", "a package searched, never past its end"),
    run(0x8E, "ObjectType", REF),
    run(0x96, "ToBuffer", TEXT),
    run(0x97, "ToDecimalString", TEXT),
    run(0x98, "ToHexString", TEXT),
    run(0x99, "ToInteger", TEXT),
    run(0x9C, "ToString", TEXT),
    run(0x9D, "CopyObject", "a copy, counted against the allocation budget"),
    run(0x9E, "Mid", TEXT),
    run(0x5B12, "CondRefOf", REF),
    run(0x5B28, "FromBCD", INT),
    run(0x5B29, "ToBCD", INT),
];
