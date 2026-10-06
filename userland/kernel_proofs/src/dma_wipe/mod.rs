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

/*
 * The DMA grant table as the shutdown wipe walks it. The real table and grant
 * types are included by path.
 */

#[path = "../../../../src/hardware/broker/dma/drain.rs"]
#[allow(dead_code)]
pub mod drain;
#[path = "../../../../src/hardware/broker/dma/records.rs"]
#[allow(dead_code)]
pub mod records;
mod tests;
#[path = "../../../../src/hardware/broker/dma/types.rs"]
#[allow(dead_code)]
pub mod types;
