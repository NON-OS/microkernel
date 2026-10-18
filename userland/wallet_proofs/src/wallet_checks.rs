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

//! The host tests of the wallet's own files.

#[path = "accounts_file_tests.rs"]
mod accounts_file_tests;
#[path = "address_text_tests.rs"]
mod address_text_tests;
#[path = "amount_tests.rs"]
mod amount_tests;
#[path = "fees_tests.rs"]
mod fees_tests;
#[path = "fetch_tests.rs"]
pub(crate) mod fetch_tests;
#[path = "format_typed_tests.rs"]
mod format_typed_tests;
#[path = "gather_tests.rs"]
mod gather_tests;
#[path = "keep_tests.rs"]
mod keep_tests;
#[path = "kept_names_tests.rs"]
mod kept_names_tests;
#[path = "layout_tests.rs"]
mod layout_tests;
#[path = "mul_div_tests.rs"]
mod mul_div_tests;
#[path = "network_tests.rs"]
mod network_tests;
#[path = "probe_model.rs"]
mod probe_model;
#[path = "replace_tests.rs"]
mod replace_tests;
#[path = "route_tests.rs"]
mod route_tests;
#[path = "scale_tests.rs"]
mod scale_tests;
#[path = "send_tests.rs"]
mod send_tests;
#[path = "sent_tests.rs"]
mod sent_tests;
#[path = "shield_typed_tests.rs"]
mod shield_typed_tests;
#[path = "stakeable_tests.rs"]
mod stakeable_tests;
#[path = "swap_curve_tests.rs"]
mod swap_curve_tests;
#[path = "swap_limits_tests.rs"]
mod swap_limits_tests;
#[path = "tests.rs"]
mod tests;
#[path = "vault_read/mod.rs"]
mod vault_read;
