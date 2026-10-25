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

//! The memory type a MkMmioMap mapping gets (L17): write-combined only on a
//! prefetchable BAR, wholly inside it, and only once every CPU's PAT has the
//! entry; strong UC otherwise. The real source is included by path.

#[path = "../../../../src/hardware/broker/mmio/cache.rs"]
pub mod cache;
mod glue_tests;
mod tests;
