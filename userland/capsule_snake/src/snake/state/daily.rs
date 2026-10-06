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

use super::difficulty::{self, Difficulty};
use super::mode::{self, Mode};

pub const DAY_MS: i64 = 86_400_000;

// The day's run: the mode turns over every day and the difficulty every four
// days, so all sixteen pairs come round in sixteen days. It is read from the
// wall clock the game already keeps (`Game::last_ms`), and the home card and
// its click both ask here, so the card names the run a click sets up.
pub fn pick(wall_ms: i64) -> (Mode, Difficulty) {
    let day = (wall_ms.max(0) / DAY_MS) as usize;
    let mode = mode::ALL[day % mode::ALL.len()];
    let level = difficulty::ALL[(day / mode::ALL.len()) % difficulty::ALL.len()];
    (mode, level)
}
