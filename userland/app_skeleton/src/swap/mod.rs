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

//! A window's swapchain: the source `nonos_surface` ships, included as it
//! stands so the toolkit drives the same chain the compositor proofs hold
//! to its rules, without a crate app_skeleton's apps would each have to
//! lock. Its fence calls go through libc here (`fence`).

mod fence;
#[path = "../../../nonos_surface/src/present.rs"]
mod present;
#[path = "../../../nonos_surface/src/swap_driver.rs"]
mod swap_driver;
#[path = "../../../nonos_surface/src/swapchain.rs"]
pub mod swapchain;
#[path = "../../../nonos_surface/src/swapchain_damage.rs"]
pub mod swapchain_damage;

pub use present::{
    on_layer, parts_block, with_parts, Part, MAX_PARTS, OP_PRESENT, OP_PRESENT_LAYER,
    OP_PRESENT_LAYER_PARTS, PARTS_LEN, PRESENT_LAYER_LEN, PRESENT_LAYER_PARTS_LEN, PRESENT_LEN,
};
pub use swap_driver::{Pending, Swap};
pub use swapchain::Chain;
pub use swapchain_damage::{Damage, Stale};
