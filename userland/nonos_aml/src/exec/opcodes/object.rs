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

//! Named objects and namespace modifiers (sections 20.2.5.1 and 20.2.5.2):
//! defined at load, or by a method as it runs.

use super::{noop, run, OpRow};

const FIELD: &str = "a field, reached only as its region's space allows (F-102)";
const BUFFER_FIELD: &str = "a field over a buffer, checked against its length";
const NODE: &str = "a namespace node, its children defined inside it";

pub const ROWS: &[OpRow] = &[
    run(0x06, "Alias", "a second name for an object that exists"),
    run(0x08, "Name", "a name and its data object"),
    run(0x10, "Scope", "opens a scope that exists"),
    run(0x14, "Method", "its body parsed at the first call; calls nest to the depth budget"),
    noop(0x15, "External", "declares only: it defines nothing"),
    run(0x8A, "CreateDWordField", BUFFER_FIELD),
    run(0x8B, "CreateWordField", BUFFER_FIELD),
    run(0x8C, "CreateByteField", BUFFER_FIELD),
    run(0x8D, "CreateBitField", BUFFER_FIELD),
    run(0x8F, "CreateQWordField", BUFFER_FIELD),
    run(0x5B13, "CreateField", BUFFER_FIELD),
    run(0x5B01, "Mutex", "acquired only at or above the sync level"),
    run(0x5B02, "Event", "signalled and waited on, every wait bounded"),
    run(0x5B80, "OperationRegion", "reached only as its space's row allows (F-102)"),
    run(0x5B81, "Field", FIELD),
    run(0x5B86, "IndexField", FIELD),
    run(0x5B87, "BankField", FIELD),
    run(0x5B82, "Device", NODE),
    run(0x5B83, "Processor", "deprecated since ACPI 6.4, still what firmware declares"),
    run(0x5B84, "PowerResource", "its _ON and _OFF counted by the device power code"),
    run(0x5B85, "ThermalZone", NODE),
    run(0x5B88, "DataTableRegion", "a firmware table the host finds and reads"),
];
