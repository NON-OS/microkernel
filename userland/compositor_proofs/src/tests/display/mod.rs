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

//! What the compositor decides about displays: polls, sizes, scales and the cursor plane.

mod arrange_tests;
mod backoff_held_tests;
mod backoff_tests;
mod bootfb_gone_tests;
mod brightness_get_tests;
mod brightness_output_tests;
mod brightness_tests;
mod cursor_plane_tests;
mod cursor_rule_tests;
mod first_frame_tests;
mod floor_tests;
mod head_tests;
mod hint_tests;
mod input_list_lie_tests;
mod input_list_tests;
mod layout_tests;
mod lid_paint;
mod lid_rig;
mod lid_rule_tests;
mod lid_session_tests;
mod lid_tests;
mod lookup_log_tests;
mod notice_tests;
mod poll_tests;
mod present_retry_tests;
mod resize_fit_tests;
mod resize_tests;
mod return_poll_tests;
mod return_tests;
mod scale_damage_tests;
mod scale_tests;
