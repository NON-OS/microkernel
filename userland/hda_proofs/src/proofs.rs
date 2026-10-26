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

//! The proofs, one module per property under test, gathered under a module so
//! no file in this crate runs past the tree's line budget. Each module's header
//! names the part of the HD Audio specification it holds the driver to, and
//! what goes wrong on a real machine when the driver stops holding to it.

mod amp_index_tests;
mod amp_route_tests;
mod at_start;
mod bdl_tests;
mod codec_refusal_tests;
mod codec_sweep_tests;
mod codec_tests;
mod conn_tests;
mod corb_start_tests;
mod corb_tests;
pub mod fixtures;
mod format_tests;
mod intel_link_tests;
mod intel_pci_tests;
mod intel_tests;
mod irq_plan_tests;
mod jack_schedule_tests;
mod jack_tests;
mod layout_tests;
pub mod lifecycle;
mod order_corb_tests;
mod order_tests;
mod output_status_tests;
mod path_budget_tests;
mod path_loop_tests;
mod period_count_tests;
mod plan_path_tests;
mod plan_tests;
mod position_hostile_tests;
mod position_tests;
mod program_amp_tests;
mod program_external_tests;
mod program_mixer_tests;
pub mod program_sim;
mod program_tests;
mod reason_tests;
mod reset_tests;
mod ring;
mod route_sel_tests;
mod sst_tests;
mod stream_dma_tests;
mod stream_irq_tests;
mod stream_tests;
mod vendor;
mod verb_answer_tests;
mod verb_idle_tests;
mod verb_payload_tests;
mod verb_ring_tests;
mod verb_tests;
mod verb_timeout_tests;
mod verb_word_tests;
mod verdict_tests;
mod wait_spin_tests;
mod walk_tests;
