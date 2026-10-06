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

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use crate::snake::state::kept;
use crate::snake::state::Game;

use super::{decode, gate, paths};

// Career state only, called once before the first frame, never per tick. An
// absent VFS, a missing file or a corrupt record leave the constructed
// defaults in place so the game is playable either way; any of them but a
// missing file (the first run) is kept in `game.kept` for the Ranks screen.
pub fn load_into(game: &mut Game) {
    if !gate::live() {
        return;
    }
    let pid = mk_getpid();
    if pid == 0 {
        return;
    }
    let ranks = match read(pid, paths::RANKS) {
        Ok(bytes) => match decode::runs(&bytes) {
            Ok(runs) => {
                game.runs = runs;
                game.runs.sort_by(|a, b| b.score.cmp(&a.score));
                kept::of_read(Ok(()), true)
            }
            Err(_) => kept::of_read(Ok(()), false),
        },
        Err(err) => kept::of_read(Err(err), false),
    };
    let awards = match read(pid, paths::AWARDS) {
        Ok(bytes) => match decode::awards(&bytes) {
            Ok(awards) => {
                game.awards = awards;
                kept::of_read(Ok(()), true)
            }
            Err(_) => kept::of_read(Ok(()), false),
        },
        Err(err) => kept::of_read(Err(err), false),
    };
    game.kept = kept::worse(ranks, awards);
}

// The vfs server rejects a claimed owner pid that differs from the real sender
// pid, so the pid is always this window's own and never a service lookup.
fn read(pid: u32, path: &[u8]) -> Result<Vec<u8>, &'static str> {
    vfs::read_file(pid, path, paths::MAX_FILE_BYTES).map_err(|err| {
        gate::note(err);
        err
    })
}
