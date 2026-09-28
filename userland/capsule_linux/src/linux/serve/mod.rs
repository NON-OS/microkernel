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

//! Answering for a guest: the loop, and the table it answers from.

mod answer;
mod deliver;
mod deliver_enter;
mod deliver_interrupt;
mod deliver_pipe;
mod deliver_rem;
mod deliver_restart;
mod deliver_say;
mod deliver_sigwait;
mod deliver_stack;
mod deliver_wait;
mod dispatch;
mod family;
mod family_futex;
mod family_lend;
mod family_reap;
mod family_reap_end;
mod family_signal;
mod family_signal_fire;
mod family_signal_route;
mod family_sleep;
mod family_view;
mod family_wait;
mod family_wait_report;
mod family_wait_try;
mod family_waits;
mod loop_impl;
mod pid_map;
mod pid_ns;
mod pid_out;
mod pid_space;
mod refused;
mod route_life;
mod table;
mod table_file;
mod table_link;
mod table_mem;
mod table_meta;
mod table_net;
mod table_proc;
mod table_sig;
mod tally;
mod unserved;
mod waits;
mod waits_fds;
mod waits_time;

pub use answer::Answer;
pub use loop_impl::serve;
pub use pid_space::{inward as kernel_pid, outward as guest_pid};
