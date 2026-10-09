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

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;
use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

use crate::snake::state::kept::{self, Kept};
use crate::snake::state::Game;

use super::{codec, encode, gate, paths};

// Written once, at the transition into Over, never per tick. A failed step
// does not stop the game: the run stays in memory for this window's lifetime
// and nothing on the frame path waits on the outcome. What became of the save
// is handed back so the Ranks screen can say it.
pub fn save_from(game: &Game) -> Kept {
    if !gate::live() {
        return Kept::NotSaved(kept::NO_SERVICE);
    }
    let pid = mk_getpid();
    if pid == 0 {
        return Kept::NotSaved("this window has no process id");
    }
    if let Err(err) = vfs::mkdir(pid, paths::DIR) {
        gate::note(err);
    }
    if !gate::live() {
        return Kept::NotSaved(kept::NO_SERVICE);
    }
    let keeps = keeps_state();
    let ranks = write(pid, paths::RANKS, &encode::runs(&game.runs), keeps);
    let held = game.awards.len().min(codec::MAX_AWARDS);
    let awards = write(pid, paths::AWARDS, &encode::awards(&game.awards[..held]), keeps);
    kept::worse(ranks, awards)
}

// Whether this boot keeps what is written, as setup chose: the policy store
// holds it, and no answer is taken as amnesic, as vfs takes it.
fn keeps_state() -> bool {
    lookup().and_then(|port| get_bool(port, Field::Persistent)) == Some(true)
}

// The owner pid is this window's own: the vfs server rejects a claimed owner
// pid that differs from the real sender pid.
fn write(pid: u32, path: &[u8], data: &[u8], keeps: bool) -> Kept {
    if let Err(err) = vfs::write_file(pid, path, data) {
        gate::note(err);
        return kept::of_file(Err(err), Ok(()));
    }
    // A live session keeps it for this session. Asking vfs to keep it anyway
    // was refused, and printed "[VFS] refused persist: amnesic boot" on the
    // serial console twice every game over.
    if !keeps {
        return Kept::Session(kept::AMNESIC);
    }
    let persisted = vfs::persist(pid, path);
    if let Err(err) = persisted {
        gate::note(err);
    }
    kept::of_file(Ok(()), persisted)
}
