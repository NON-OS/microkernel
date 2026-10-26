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
 * A GPU driver's claim taking the boot framebuffer's scanout (L79): its pure
 * rules included by path and tested cell by cell, an exhaustive model of the
 * takeover racing bootfb's claim, map, unmap, release and exit and a kernel
 * draw, each design step undone in the model, and the glue read as text.
 */

mod admit_table_tests;
mod control_tests;
mod drawer;
mod events;
mod exit;
mod found;
mod give_back;
mod glue_claim_tests;
mod glue_console_tests;
mod glue_enter_tests;
mod glue_take_tests;
mod glue_text;
mod glue_unmap_tests;
mod holder;
mod holder_admit;
mod holder_map;
mod knob;
mod model_tests;
mod mutation_count_tests;
mod mutation_order_tests;
mod mutation_page_tests;
mod pinned;
mod repoint;
mod rule_tests;
#[path = "../../../../src/hardware/broker/claim/scanout_rule.rs"]
mod scanout_rule;
mod search;
mod sibling;
mod start_sets;
mod starts;
mod steps;
mod take_back;
mod take_order;
mod taker;
mod taker_admit;
mod taker_peek;
mod world;
mod world_ask;
mod world_check;
mod world_edit;
mod world_names;
mod world_state;

// The claims' stage rule and types, mounted once, in claim_stage.
use crate::proofs_7::claim_stage::{stage_rule, types};
