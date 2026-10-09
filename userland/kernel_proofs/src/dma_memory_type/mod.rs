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
 * The memory types a DMA grant's user mapping lands on: a coherent grant is
 * strong uncached, a write-combining one is WC once the PAT has the entry
 * and uncached before then, and every other grant stays write-back. The PAT
 * value and the broker's flags are included by path.
 */

#[path = "../../../../src/hardware/broker/dma/flags.rs"]
#[allow(dead_code)]
pub mod flags;
mod tests;
#[path = "../../../../src/arch/x86_64/pat/value.rs"]
#[allow(dead_code)]
pub mod value;
