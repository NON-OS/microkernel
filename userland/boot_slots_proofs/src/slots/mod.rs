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

//! The kernel's `security::boot::slots` files that need no kernel, at the
//! paths their `super::` imports expect, with the tests beside them.

#[path = "../../../../src/security/boot/slots/footer.rs"]
mod footer;
#[path = "../../../../src/security/boot/slots/record.rs"]
mod record;
#[path = "../../../../src/security/boot/slots/slot.rs"]
mod slot;

mod footer_mutation_tests;
mod footer_tests;
mod record_tests;
mod slot_tests;
