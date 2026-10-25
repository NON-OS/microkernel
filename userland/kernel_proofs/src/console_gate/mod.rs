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
 * The console's draw gate (L79): a kernel draw counted in DRAWS while it
 * writes the framebuffer, and stopped by HELD, by GONE or by a takeover
 * under way. The gate reaches kernel globals and cannot be mounted, so a
 * model whose steps are its atomic operations one for one is searched over
 * every interleaving, and the kernel's text is read to check that it takes
 * those steps in that order.
 */

mod actors;
mod check;
mod defect_tests;
mod drawer;
mod drawer_steps;
mod events;
mod fidelity_tests;
mod gate;
mod gates;
mod glue_claim_tests;
mod glue_gate_tests;
mod glue_held_tests;
mod glue_hold_tests;
mod glue_names_tests;
mod glue_pin_drawing_tests;
mod glue_pin_held_tests;
mod glue_reach_tests;
mod glue_screens_tests;
mod glue_sites_tests;
mod glue_takeover_tests;
mod glue_writers_tests;
mod hold_steps;
mod model_tests;
mod moves;
mod mutation_tests;
mod order_tests;
mod others;
mod outcome;
mod search;
mod start;
mod text;
mod text_items;
mod text_uses;
mod world;
