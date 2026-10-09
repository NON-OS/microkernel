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

pub const COLS: i16 = 35;
pub const ROWS: i16 = 21;

// The nominal window, matching capsule_process_manager, whose HiDPI behaviour
// is boot-proven. There is no fixed cell size: the board is fitted to the live
// surface every frame (paint/board_fit.rs), so maximize still works.
pub const WIN_W: u32 = 1240;
pub const WIN_H: u32 = 780;

// The snake is placed here on every reset, heading right. Walls are kept out
// of the rectangle around it so a level change can never spawn a kill.
pub const SPAWN: (i16, i16) = (COLS / 2, ROWS / 2);
pub const SPAWN_CLEAR_X: i16 = 7;
pub const SPAWN_CLEAR_Y: i16 = 2;
