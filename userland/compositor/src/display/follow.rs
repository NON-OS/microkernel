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

//! Decision 15, whether the display follows the host, built to the maintainers'
//! recommendation until the owner rules: follow it at start (the preferred
//! mode) and whenever it asks (a STATE hint, or a DISPLAY_HINT from a
//! hint source), so the desktop may differ from the boot console's size.
//! An owner "no" is this one line set to false: firmware's mode is kept,
//! hints are logged and answered 0, and no MODE_SET is ever sent.

pub const FOLLOW_HOST: bool = true;
