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

use super::pad::is_bti_landing_pad;

/// Whether the instruction at `at` is one a guarded branch may land on.
///
/// # Safety
///
/// `at` is a readable, aligned instruction word, such as kernel text.
pub unsafe fn check_bti_landing_pad(at: *const u32) -> bool {
    // SAFETY: the caller's.
    is_bti_landing_pad(unsafe { at.read() })
}
