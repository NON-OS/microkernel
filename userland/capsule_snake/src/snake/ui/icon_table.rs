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

use nonos_toolkit::icons::IconId;

use crate::snake::state::Mode;

// Every mark in the capsule comes from the shared icon source, so a screen's
// art is the same file the dock and the shell already carry.
pub fn mode(mode: Mode) -> IconId {
    match mode {
        Mode::Arcade => IconId::GameBolt,
        Mode::Classic => IconId::GameTarget,
        Mode::TimeAttack => IconId::GameStopwatch,
        Mode::Zen => IconId::GameLotus,
    }
}

// Ordinal-indexed against `setup_geom_rows::TOGGLE_LABELS` and against the four
// HUD cards; both tables are hand-synced with the labels they mark.
pub fn option(index: usize) -> IconId {
    const MARKS: [IconId; 3] = [IconId::GameBlocks, IconId::GameLotus, IconId::GameBolt];
    MARKS[index.min(MARKS.len() - 1)]
}

pub fn hud(index: usize) -> IconId {
    match index {
        0 => IconId::GameTrophy,
        1 => IconId::GameTarget,
        2 => IconId::GameBlocks,
        _ => IconId::GameHeart,
    }
}

pub fn award(unlocked: bool) -> IconId {
    if unlocked {
        IconId::GameCrown
    } else {
        IconId::GameLock
    }
}
