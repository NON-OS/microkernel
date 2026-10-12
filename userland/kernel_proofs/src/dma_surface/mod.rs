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

//! a DMA grant's frames that back a shared surface are held, not
//! freed, until no other process maps the surface; then given back once.
//! Before K15 the broker freed them under the compositor's mapping.

#[path = "../../../../src/kernel_core/surface_registry/pin/dma_plan.rs"]
mod dma_plan;
#[path = "../../../../src/kernel_core/surface_registry/pin/dma_rule.rs"]
mod dma_rule;

mod common;
mod exit;
mod release;
