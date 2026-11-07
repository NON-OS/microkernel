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

//! A surface lent to a display device for direct scanout
//! (src/hardware/broker/dma/import): what an import is refused for, the map
//! of its runs into a domain read only and its undo, the drains that take it
//! out, and the frame quarantine an unconfirmed unmap leaves.

#[path = "../../../../src/hardware/broker/dma/import/rule.rs"]
pub mod rule;
#[path = "../../../../src/hardware/broker/dma/import/runs.rs"]
pub mod runs;
#[path = "../../../../src/hardware/broker/dma/import/table_rule.rs"]
pub mod table_rule;
#[path = "../../../../src/memory/phys/quarantine_rule.rs"]
pub mod quarantine_rule;

#[cfg(test)]
mod model;
#[cfg(test)]
mod tests;
