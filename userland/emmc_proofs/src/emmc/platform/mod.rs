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

//! The capsule's platform glue: discovery's record, the claim and open path
//! and the DMA regions as the capsule builds them, over the stand-in
//! nonos_libc, and a register window (`env`) that reaches the model host.

mod discover;
#[path = "../../../../capsule_driver_sdhci/src/emmc/platform/dma.rs"]
mod dma;
mod env;
#[path = "../../../../capsule_driver_sdhci/src/emmc/platform/open/mod.rs"]
mod open;

pub use discover::Found;
pub use open::{open, open_removable, Opened};
