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

//! The proofs. The checks many of them share are in `check`.

mod attest;
mod attest_echo;
mod attest_pcrs;
mod auth;
mod auth_grant;
mod check;
mod cli;
mod cli_bind;
mod cli_kit;
mod cli_usage;
mod enrol;
mod enrol_time;
mod fuzzed;
mod hello;
mod hello_raw;
mod host_sig;
mod host_sig_body;
mod login;
mod login_keys;
mod order;
mod order_open;
mod own;
mod record_box;
mod record_ckpt;
mod record_ckpt_forged;
mod record_flip;
mod record_good;
mod record_header;
mod record_own;
mod record_own_cut;
mod record_revoked;
mod record_tamper;
mod session;
mod session_end;
mod session_end_order;
mod session_escalate;
mod session_send;
mod session_send_rules;
mod signer;
mod wire_changed;

pub use check::{aborted_with, host_refusal, host_run, silent};
