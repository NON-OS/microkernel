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

//! The proofs, one module per stage of the host side.

#[path = "../../../capsule_nsh_front/src/limits.rs"]
mod front_limits;
mod admit;
mod admit_answer;
mod admit_begin;
mod admit_frames;
mod cli;
mod cli_close;
mod cli_explain;
mod cli_step5;
mod cli_files;
mod cli_releases;
mod devkit;
mod devkit_live;
mod devkit_cli;
mod devkit_host;
mod escalate;
mod escalate_ends;
mod escalate_host;
mod escalate_one;
mod escalate_refused;
mod good;
mod good_run;
mod hello;
mod hello_host;
mod hello_kernel;
mod hello_limits;
mod hello_order;
mod hello_silent;
mod login;
mod login_store;
mod login_table;
mod loopback;
mod loopback_clock;
mod loopback_dial;
mod loopback_got;
mod loopback_host;
mod loopback_refused;
mod loopback_shell;
mod loopback_suite;
mod loopback_users;
mod loopback_wire;
mod noise;
mod noise_flipped;
mod probe;
mod record;
mod record_cap;
mod record_keys;
mod record_store;
mod session;
mod session_calls;
mod session_ends;
mod session_expiry;
mod session_frames;
mod session_longest;
mod session_record;
mod session_roles;
mod session_shell;
mod store;
mod store_session;
mod turned_away;
