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

//! The volume and power keys in the shell: the codes it answers are the ones
//! the router sends it, a press steps and clamps the level or flips mute and
//! asks the audio service for exactly that, and the notice says what plays
//! in one toast that the next press replaces.

mod codes;
mod held;
mod notices;
mod request;
mod rig;
mod steps;
