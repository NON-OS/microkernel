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

/*
 * The wallet's side of Shield. The shield wallet itself, the phones' own
 * shield-core with its prover, runs as the nonos.shield service; this is
 * the client that opens it for this account, asks it for reviews and
 * proofs, and follows what it reports.
 */

pub mod actions;
mod actions_confirm;
mod actions_withdraw;
mod apply;
mod apply_confirmed;
mod apply_open;
mod apply_proved;
mod apply_refresh;
mod apply_review;
mod apply_state;
mod apply_values;
pub mod client;
pub mod due;
mod due_entry;
pub mod due_feed;
mod due_why;
mod follow;
mod follow_due;
mod follow_earlier;
mod follow_older;
pub mod fresh_move;
pub mod fresh_move_act;
pub mod fresh_move_job;
pub mod fund_meter;
pub mod fund_order;
pub mod gas_withdraw;
pub mod job;
mod job_poll;
mod job_progress;
mod job_resume;
mod job_start;
mod job_steps;
mod kept_names;
pub mod landed;
mod landed_ask;
pub mod landed_held;
pub mod landed_poll;
pub mod open;
pub mod probe;
mod probe_lookup;
pub mod reply;
pub mod scan;
pub mod scan_job;
mod scan_state;

pub use follow::due;
