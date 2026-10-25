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

//! The tests, one module per part of the format.

mod auth_ok;
mod cap_names;
mod enrol;
mod enrol_boot;
mod frame;
mod grant;
mod grant_approvals;
mod grant_parent;
mod hello;
mod hello_preamble;
mod host_proof;
mod len_field;
mod names;
mod noise;
mod noise_parsers;
mod order;
mod reader;
mod reasons;
mod record;
mod record_box;
mod record_box_limits;
mod record_box_source;
mod record_command;
mod record_entries;
mod record_footer;
mod session;
mod session_replies;
mod sizes;
mod stages;
mod user_auth;
mod writer;
