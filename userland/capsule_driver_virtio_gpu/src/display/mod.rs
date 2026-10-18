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
//! virtio-gpu on the display wire: its enabled scanouts are the
//! outputs, each showing a surface of the driver's own (DMA it maps, a 2D
//! resource on the host), and `nonos_gfx_serve` answers the compositor.

pub mod describe;
pub mod events;
pub mod gpu;
pub mod limbo;
pub mod mode_set;
pub mod modes;
pub mod plug;
pub mod reshow;
pub mod show;
pub mod surface;
pub mod take;
pub mod wire;

pub use gpu::Gpu;
