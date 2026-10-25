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

//! The send tests: accounts, pasted addresses, amounts, fees, the sent record, sends.

#[path = "accounts_file_tests.rs"]
mod accounts_file_tests;
#[path = "address_paste_tests.rs"]
mod address_paste_tests;
#[path = "address_text_tests.rs"]
mod address_text_tests;
#[path = "amount_edit_tests.rs"]
mod amount_edit_tests;
#[path = "amount_limits_tests.rs"]
mod amount_limits_tests;
#[path = "amount_tests.rs"]
mod amount_tests;
#[path = "fees_tests.rs"]
mod fees_tests;
#[path = "mul_div_tests.rs"]
mod mul_div_tests;
#[path = "send_broadcast_tests.rs"]
mod send_broadcast_tests;
#[path = "send_refusal_tests.rs"]
mod send_refusal_tests;
#[path = "send_tests.rs"]
mod send_tests;
#[path = "sent_tests.rs"]
mod sent_tests;
