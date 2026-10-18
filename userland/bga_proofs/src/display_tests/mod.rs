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

//! The driver up and serving: the display wire the compositor speaks to it,
//! held to virtio-gpu's, the mode kept from firmware and written back, and
//! the name it is found under, beside the compositor's own order of asking.

/// The clamping adapter, which setup_confirm_tests drives too.
pub(crate) mod clamp;
mod edge_tests;
mod firmware_tests;
mod flush_tests;
mod frames;
#[path = "../../../compositor/src/setup/discover/order.rs"]
mod gfx_order;
mod left;
mod list_tests;
mod model;
mod not_taken_tests;
mod op_tests;
mod owner_tests;
mod picture_tests;
mod primary_tests;
mod rects;
mod resource_tests;
mod respawn_tests;
mod restore_tests;
mod same_wire_tests;
mod scanout_tests;
mod service_tests;
mod surface_tests;
mod turn_tests;
mod video_tests;
/// The wire as virtio-gpu frames it, whole: its numbers, header and codec.
#[path = "../../../capsule_driver_virtio_gpu/src/protocol/mod.rs"]
pub mod virtio_wire;
mod wire_tests;
mod write_back_tests;
