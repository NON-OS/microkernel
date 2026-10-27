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

//! Proofs of the calls a guest makes: memory, signals, files, sockets,
//! processes and the box's rules.

mod anon_backend_tests;
mod anon_reply_tests;
mod anon_rx_tests;
mod app_choose_tests;
mod app_identity_tests;
mod app_net_step_tests;
mod app_reach_exec_tests;
mod app_reach_tests;
mod app_reach_write;
mod app_rules_tests;
mod bind_any_tests;
mod applets_tests;
mod clone_flag_tests;
mod console_tests;
mod dir_children_tests;
mod dirent_tests;
mod elf_tests;
mod exe_table_tests;
mod exec_args_tests;
mod exec_shebang_tests;
mod fake_memory;
mod family_tests;
mod fd_flag_tests;
mod futex_tests;
mod gram_tests;
mod guest_limits_tests;
mod guest_route_tests;
mod ioctl_tests;
mod iovec_tests;
mod key_tests;
mod link_tests;
mod load_tests;
mod lock_tests;
mod mem_args_tests;
mod metrics_parse_tests;
mod mutation;
mod mutation_tests;
mod path_tests;
mod private_rule_tests;
mod punch_tests;
mod quota_tests;
mod ready_tests;
mod resolve_tests;
mod resolver_msg_tests;
mod route_tests;
mod run_mode_tests;
mod sigframe_layout_tests;
mod sigframe_mutation_tests;
mod sigframe_tests;
mod signal_rule_tests;
mod sigtimer_tests;
mod stack_words_tests;
mod stat_size_tests;
mod stat_tests;
mod tasks_tests;
mod wait_option_tests;
mod walk_tests;
