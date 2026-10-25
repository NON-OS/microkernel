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
//! 05  SEND, a public payment from this account: the form, the review of
//! exactly what will be signed, and what the network said once it went.

pub mod click;
mod click_all;
mod click_step;
mod done;
mod done_said;
mod form;
mod form_fields;
pub mod key;
mod key_address;
mod review;
mod review_rows;
mod show;

pub use show::show;
