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

//! The screen a boxed run's report is drawn on.

/// What the screen is put back to before the report is drawn, whatever the
/// guest left (an escape it left half-written ends at the first ESC here):
/// the main screen, first, since leaving the alternate one restores the
/// cursor saved on entering it; a soft reset (DECSTR: region, origin,
/// insert, autowrap, charsets and pen); newline mode and reverse video off,
/// which DECSTR keeps; and a fresh line at the screen's foot, so the report
/// is the last thing drawn and scrolls into history whole.
pub const SCREEN_RESET: &[u8] = b"\x1b[?1049l\x1b[!p\x1b[20l\x1b[?5l\x1b[999;1H\r\n";
