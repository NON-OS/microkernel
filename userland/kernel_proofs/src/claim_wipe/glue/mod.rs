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


//! The kernel glue the models stand for (L78, L80), read from its source with
//! its comments taken out (code) and its functions pinned whole (whole), and
//! the rules beside it: a kept run's way back and its count (kept_world),
//! and Device Control 2 put back.

mod bits_ct_tests;
mod code;
mod code_tests;
mod entry_pin_tests;
mod finish_pin_tests;
mod glue_l80_tests;
mod glue_l80b_tests;
mod glue_libc_tests;
mod glue_tests;
mod inboxes;
mod kept_count_tests;
mod kept_pin_tests;
mod kept_tests;
mod kept_world;
mod teardown_pin_tests;
mod whole;
mod whole_tests;
