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

//! The screens a hostile guest leaves behind it, for box_screen_tests.

/// What a guest leaves: each one is a way the review found to hide or bend
/// the report, one a fake report drawn where the real one would go.
pub const LEFT: [&str; 19] = [
    "\x1b[23;24r\x1b[24;1H",
    "\x1b)0\x0e",
    "\x1b(0",
    "\x1b[?6h\x1b[1;2r",
    "\x1b[4h",
    "\x1b[?7l",
    "\x1b[20h",
    "\x1b[?5h",
    "\x1b]0;swallowed by an unended title",
    "\x1bPswallowed by an unended device string",
    "\x1b_swallowed by an unended application string",
    "\x1b[1;2;3",
    "\x1bN",
    "\x1b(",
    "\x1b[?1049h",
    "\x1b)0\x0e\x1b[?6h\x1b[2;3r\x1b7",
    "\x1b[2J\x1b[1;1HBOX REPORT v1\r\n  refused  0\r\n\x1b[H",
    "\x1b[24;1Hthe guest's own last line, longer than any line of the report",
    "\x1b[2;1Hlong rows left below the cursor, to run on into the report's lines\r\n\
     long rows left below the cursor, to run on into the report's lines\x1b[H",
];
