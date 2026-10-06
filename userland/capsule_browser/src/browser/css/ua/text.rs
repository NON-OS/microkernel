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

/* Text-level defaults: link colour (#0000ee, Chromium's -webkit-link),
 * emphasis, code faces, sizes and decorations. */
pub(super) const SHEET: &str = concat!(
    "a[href]{color:#0000ee;text-decoration:underline}",
    "i,cite,em,var,dfn,address{font-style:italic}",
    "code,kbd,samp,tt,pre,listing,xmp,plaintext{font-family:monospace}",
    "pre,listing,xmp,plaintext{white-space:pre}",
    "textarea{white-space:pre-wrap}",
    "nobr{white-space:nowrap}",
    "small,sub,sup{font-size:smaller}big{font-size:larger}",
    "u,ins{text-decoration:underline}s,strike,del{text-decoration:line-through}",
    "abbr[title],acronym[title]{text-decoration:underline dotted}",
    "mark{background-color:#ffff00;color:#000000}",
);
