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

#![no_std]

mod attach;
mod create;
mod damage;
mod destroy;
mod doorbell;
pub mod fence;
mod present;
mod ring;
mod ring_error;
mod ring_header;
mod ring_sync;
mod share;
mod swap_driver;
pub mod swapchain;
mod swapchain_damage;
mod types;

pub use attach::attach;
pub use create::create;
pub use damage::damage;
pub use destroy::destroy;
pub use doorbell::{Armed, Bell};
pub use present::{
    on_layer, parts_block, with_parts, Part, Present, INDEX_OPAQUE, MAX_PARTS, OP_PRESENT,
    OP_PRESENT_LAYER, OP_PRESENT_LAYER_PARTS, PARTS_LEN, PRESENT_LAYER_LEN,
    PRESENT_LAYER_PARTS_LEN, PRESENT_LEN,
};
pub use ring::{Consumer, Producer, Ring};
pub use ring_error::RingError;
pub use ring_header::RingHeader;
pub use share::{share, SHARE_WRITE};
pub use swap_driver::{Pending, Swap};
pub use swapchain::{Chain, Misuse, Slot, MAX_BUFFERS};
pub use swapchain_damage::{Damage, Stale};
pub use types::{SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};
