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

//! K15, by run since L35: a DMA run under a shared surface is held, never
//! freed, while another process maps a surface over it, and given back once
//! the last holder lets go. The kernel's pure rules are included by path;
//! the L35 path around them is modelled (common, model, let_go,
//! exit_model) and its code pinned whole (pin_run, pin_detach, pin_exit),
//! so the model cannot drift from it unseen.

#[path = "../../../../src/kernel_core/surface_registry/pin/dma_rule.rs"]
mod dma_rule;
#[path = "../../../../src/kernel_core/surface_registry/pin/reclaim_rule.rs"]
mod reclaim_rule;

mod common;
mod exit;
mod exit_model;
mod let_go;
mod model;
mod pin_detach;
mod pin_exit;
mod pin_run;
mod pin_text;
mod reclaim;
mod release;
mod threads;
