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

//! The market, meter and Shield tests: orders, intents, settle, meters, Shield, swaps.

#[path = "attest_tests.rs"]
mod attest_tests;
#[path = "closed_due_tests.rs"]
mod closed_due_tests;
#[path = "due_tests.rs"]
mod due_tests;
#[path = "eth_move_tests.rs"]
mod eth_move_tests;
#[path = "fund_meter_tests.rs"]
mod fund_meter_tests;
#[path = "held_tests.rs"]
mod held_tests;
#[path = "intent_tests/mod.rs"]
mod intent_tests;
#[path = "landed_tests.rs"]
mod landed_tests;
#[path = "meter_budget_tests.rs"]
mod meter_budget_tests;
#[path = "meter_job_tests.rs"]
mod meter_job_tests;
#[path = "meter_rows_tests.rs"]
mod meter_rows_tests;
#[path = "move_tests.rs"]
mod move_tests;
#[path = "order_tests/mod.rs"]
mod order_tests;
#[path = "publisher_tests/mod.rs"]
mod publisher_tests;
#[path = "scan_tests.rs"]
mod scan_tests;
#[path = "settle_heard_tests.rs"]
mod settle_heard_tests;
#[path = "shield_amount_tests.rs"]
mod shield_amount_tests;
#[path = "shield_fixtures.rs"]
mod shield_fixtures;
#[path = "shield_paste_tests.rs"]
mod shield_paste_tests;
#[path = "shield_typed_tests.rs"]
mod shield_typed_tests;
#[path = "stakeable_tests.rs"]
mod stakeable_tests;
#[path = "swap_curve_tests.rs"]
mod swap_curve_tests;
#[path = "swap_limits_tests.rs"]
mod swap_limits_tests;
#[path = "typed_tests/mod.rs"]
mod typed_tests;
