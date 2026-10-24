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
mod cli_tests;
mod configs;
mod fake;
mod fake_report;
mod fixture;
mod fixture_flip;
mod flip_tests;
mod judge_more_tests;
mod judge_tests;
mod kernel_footer_tests;
mod kernel_key_tests;
mod kernel_tests;
mod ledger_tests;
mod matrix_tests;
mod plan_go_tests;
mod plan_tests;
mod receipt_shape_tests;
mod receipt_tests;
mod record_shape_tests;
mod record_tests;
mod run;
mod sealed_fixture;
mod sealed_link_tests;
mod sealed_tests;
mod source_fail_tests;
mod source_tests;
mod tree_fixture;
mod tree_git;
mod verdict_tests;
