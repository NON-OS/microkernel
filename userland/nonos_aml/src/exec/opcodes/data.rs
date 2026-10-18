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

//! Data objects, locals, arguments and the Debug object (sections 20.2.3
//! and 20.2.6).

use super::{run, Fate, OpRow};

const WORD: &str = "an integer constant, cut to the table's width";
const BYTES: &str = "made data, its bytes counted against the allocation budget";

pub const ROWS: &[OpRow] = &[
    run(0x00, "Zero", WORD),
    run(0x01, "One", WORD),
    run(0x0A, "BytePrefix", WORD),
    run(0x0B, "WordPrefix", WORD),
    run(0x0C, "DWordPrefix", WORD),
    run(0x0D, "StringPrefix", BYTES),
    run(0x0E, "QWordPrefix", WORD),
    run(0x11, "Buffer", BYTES),
    run(0x12, "Package", BYTES),
    run(0x13, "VarPackage", BYTES),
    run(0xFF, "Ones", WORD),
    run(0x5B30, "Revision", "the interpreter's own revision"),
    OpRow {
        first: 0x60,
        last: 0x67,
        name: "Local0-Local7",
        fate: Fate::Run,
        why: "a frame's slots; read before a store, a fault",
    },
    OpRow {
        first: 0x68,
        last: 0x6E,
        name: "Arg0-Arg6",
        fate: Fate::Run,
        why: "the call's arguments; one never passed, a fault",
    },
    run(0x5B31, "Debug", "a store is said on the host's log; a read is refused where it stands"),
];
