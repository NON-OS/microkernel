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

/*
 * The boot framebuffer published as a claimable display device (L31): the
 * window a driver may map and the geometry `MkBootFramebuffer` answers. The
 * kernel's pure rule is included by path. Beside it, whose claim takes the
 * framebuffer's scanout (L79): the cover rule included by path, a model of
 * the find the kernel runs at boot, and the glue around both read as text,
 * the glue that decides pinned whole.
 */

#[path = "../../../../src/hardware/broker/bootfb/cover_rule.rs"]
pub mod cover_rule;
#[path = "../../../../src/hardware/broker/bootfb/rule.rs"]
pub mod rule;

mod cover_ask_glue_tests;
mod cover_bound_tests;
mod cover_byte_gen;
mod cover_byte_tests;
mod cover_edge_tests;
mod cover_find;
mod cover_gen;
mod cover_glue_tests;
mod cover_model;
mod cover_pages_tests;
mod cover_pin_tests;
mod cover_reach_tests;
mod cover_seeded_tests;
mod cover_sweep_tests;
mod cover_tests;
mod covers_fallback_tests;
mod covers_tests;
mod dma_glue_tests;
mod glue_lex;
mod glue_text;
mod map_glue_tests;
mod record_glue_tests;
mod register_glue_tests;
mod register_pin_tests;
mod seed_glue_tests;
mod table_glue_tests;
mod tests;
