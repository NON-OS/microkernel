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

//! How far the directory bootstrap has got.

/// Each stage needs the one before it to have produced something real.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Bootstrap {
    Cold,
    Anchored,
    /*
     * A live consensus names five thousand relays, and their microdescriptors
     * arrive ninety two to a request: fifty five fetches, each its own TCP
     * connection. Doing all of them before answering anything held the serve loop
     * for minutes, during which no link opened, no circuit built, and every
     * caller was told there was no directory. They are spread one to a turn now,
     * and this is the stage where that happens.
     */
    Joining,
    Ready,
}
