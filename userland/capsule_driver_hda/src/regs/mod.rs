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

//! The controller's register window: the user VA the broker mapped BAR0 at
//! (`setup::mmio`). An access through `Regs` is sound while that mapping
//! lives, as long as the `BrokerHandles` holding its grant, and when the
//! register lies inside the mapped length, aligned to its width.
//!
//! The first page always lies inside. The BAR is at least 4 KiB
//! (`HDA_BAR_MIN_SIZE`, checked at discovery), so the BAR and the window
//! start page aligned, and the broker maps a request's first page whole or
//! refuses it: an MSI-X table in the BAR cuts the mapping short only at a
//! later page (src/hardware/broker/mmio/window.rs, `window`). Every global
//! register and all 64 stream descriptors, ending at 0x880, sit in that
//! page, each at an offset aligned to its width. Past it only the length
//! the broker reported is reached (`setup::take`, `link_up`).

mod mmio;

pub use mmio::Regs;
