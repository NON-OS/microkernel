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

//! The reported case end to end: with the text editor open and focused, one
//! press on the terminal's title bar raises the terminal, focuses it and starts
//! its drag, and the drag carries it wherever the pointer goes, across where
//! its own tabs were, until the release. After every step what the screen
//! shows is a full frame's worth and is what a press there would hit.

mod dock;
mod random;
mod rig;
mod targets;
mod title;
