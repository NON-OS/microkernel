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

/* Form controls sit in the line like words, as Chromium's do: inline
 * blocks with no margin, a thin inset border and the 13.33px control
 * font; buttons are raised. Their width and height come from the
 * control's own intrinsic size where CSS leaves them auto. */
pub(super) const SHEET: &str = concat!(
    "input,button,select,textarea{display:inline-block;margin:0;padding:1px 2px;",
    "border:2px inset #767676;font-size:13.333px;background-color:#ffffff;color:#000000}",
    "button,input[type=submit],input[type=button],input[type=reset]",
    "{padding:1px 6px;border:2px outset #767676;background-color:#efefef}",
    "input[type=hidden]{display:none}",
);
