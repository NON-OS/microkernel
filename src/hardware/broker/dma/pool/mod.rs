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

//! The two reserved DMA pools: a high one for display surfaces and a low one
//! below 4 GiB for devices that name 32-bit addresses. Each tracks its pages
//! in a bitmap; a run handed out by a pool goes back to that pool only.

mod bitmap;
mod display;
mod display_free;
mod give_back;
mod low32;
mod low32_alloc;
mod low32_free;
mod pressure;
mod run_taken;
mod sizing;

pub(super) use display::alloc;
pub(crate) use display::init_display_pool;
pub(super) use give_back::give_back;
pub(crate) use low32::init_low32_pool;
#[cfg(not(target_arch = "x86_64"))]
pub(crate) use low32::low32_default_pages;
pub(super) use low32_alloc::low32_alloc;
pub(super) use pressure::say_low32;
#[cfg(target_arch = "x86_64")]
pub(crate) use sizing::{low32_fit, low32_target_pages};
