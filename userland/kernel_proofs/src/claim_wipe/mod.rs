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
 * A function wiped between holders (L78): the reset's pure rules included by
 * path and each tested at its edges, a seeded model of holders, steppers,
 * claimants and a device that may never come back, an exhaustive model of a
 * claim's entry racing the wipe its last holder owes, and each design rule
 * undone to show the proofs would catch it.
 */

#[path = "../../../../src/hardware/broker/claim/wipe/admit_rule.rs"]
mod admit_rule;
mod admit_tests;
mod bits_const_tests;
mod bits_ready_tests;
#[path = "../../../../src/hardware/broker/claim/wipe/bits_rule.rs"]
mod bits_rule;
mod bits_tests;
#[path = "../../../../src/hardware/broker/claim/wipe/due_rule.rs"]
mod due_rule;
#[path = "../../../../src/hardware/broker/claim/wipe/end_rule.rs"]
mod end_rule;
mod enter_act;
mod enter_admit;
mod enter_again;
mod enter_check;
mod enter_claim;
mod enter_clock;
mod enter_ends_tests;
mod enter_events;
mod enter_fates;
mod enter_hash;
mod enter_leave;
mod enter_mutation_tests;
mod enter_parts;
mod enter_record;
mod enter_search;
mod enter_settle;
mod enter_stage;
mod enter_steps;
mod enter_tests;
mod enter_wait;
mod enter_wipe;
mod enter_world;
mod glue;
mod holder_pool;
#[path = "../../../../src/hardware/broker/claim/wipe/holder_rule.rs"]
mod holder_rule;
mod holder_tests;
#[path = "../../../../src/hardware/broker/dma/kept_rule.rs"]
mod kept_rule;
#[path = "../../../../src/hardware/broker/claim/wipe/marker_rule.rs"]
mod marker_rule;
mod method_fallback_tests;
#[path = "../../../../src/hardware/broker/claim/wipe/method_rule.rs"]
mod method_rule;
mod method_tests;
mod model_act;
mod model_admit;
mod model_attempt;
mod model_built_tests;
mod model_check;
mod model_claim;
mod model_end;
mod model_enter;
mod model_exit;
mod model_finish;
mod model_floors;
mod model_hold;
mod model_leave;
mod model_movable;
mod model_move;
mod model_mutation_tests;
mod model_reset;
mod model_settle;
mod model_stage;
mod model_step;
mod model_stepper;
mod model_sweep;
mod model_table;
mod model_tenancy;
mod model_tests;
mod phase_deadline_tests;
mod phase_due_tests;
#[path = "../../../../src/hardware/broker/claim/wipe/phase_rule.rs"]
mod phase_rule;
mod phase_tests;
#[path = "../../../../src/hardware/broker/claim/wipe/start_rule.rs"]
mod start_rule;
mod step_cases;
mod step_checked_tests;
mod step_ended_tests;
mod step_tests;
mod steps;
mod world;
mod world_cases;
mod world_check;
mod world_dev;
mod world_free;
mod world_function;
mod world_kept;
mod world_knob;
mod world_new;
mod world_regs;
mod world_reset;
mod world_runs;
mod world_thread;
mod world_types;
mod world_writes;
