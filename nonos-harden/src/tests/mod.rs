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

//! The proofs, run with `cargo test`. Each states one property of the verdict
//! in its name; the mutants each one catches are listed in
//! docs/release/nonos-harden.md.

mod attest;
mod boot_gate_tests;
mod cli_tests;
mod configs;
mod declared_tests;
mod fake;
mod fake_report;
mod fixture;
mod fixture_flip;
mod flip_tests;
mod fs_tests;
mod fuzz;
mod fuzz_genuine_tests;
mod fuzz_kernel_tests;
mod fuzz_noise_tests;
mod fuzz_text_tests;
mod judge_more_tests;
mod judge_tests;
mod kernel_bind_tests;
mod kernel_footer_tests;
mod kernel_key_tests;
mod kernel_tests;
mod kind_tests;
mod ledger_tests;
mod limits_tests;
mod matrix_tests;
mod mutants_tests;
mod plan_go_tests;
mod plan_tests;
mod receipt_shape_tests;
mod receipt_tests;
mod record_shape_tests;
mod record_tests;
mod run;
mod runner_group_tests;
mod runner_tests;
mod sealed_fixture;
mod sealed_link_tests;
mod sealed_tests;
mod sign_refuse_more_tests;
mod sign_refuse_tests;
mod sign_tamper_tests;
mod sign_tests;
mod signing;
mod source_fail_tests;
mod source_hide_tests;
mod source_keys_tests;
mod source_tests;
mod tree_choice_tests;
mod tree_fixture;
mod tree_git;
mod verdict_tests;
