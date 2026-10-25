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

//! The proofs, one module each, over the code the crate root mounts.

mod brightness_tests;
mod clock_line_tests;
mod desktop_files_tests;
mod dialog_keys_tests;
mod dock_clock_tests;
mod dock_tests;
mod focus_child_tests;
mod go_menu_tests;
mod grab_rule_tests;
mod instance_tests;
mod launch_tests;
mod launchpad_wheel_tests;
mod lid_tests;
mod live_prompt_tests;
mod log_line_tests;
mod min_size_tests;
mod open_arg_tests;
mod paste_line_tests;
mod patience_tests;
mod quiet_gap_tests;
mod relaunch_tests;
mod says_tests;
mod shadow_reach_tests;
mod shell_scale_tests;
mod store_word_tests;
mod system_key_tests;
// The apps' shared text input: which key types what into a field, which
// key pastes, and what a pasted line becomes.
mod text_paste_tests;
mod text_typed_tests;
mod toast_generation_tests;
mod toast_lifetime_tests;
mod tray_fit_tests;
mod tray_share_tests;
mod valid_str_tests;
mod vfs_budget_tests;
mod whole_frame_tests;
