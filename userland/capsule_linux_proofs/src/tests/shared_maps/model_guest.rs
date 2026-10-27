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

//! What a guest of the model does on its own: maps a file shared, read-only,
//! through a descriptor open to write, or writable, anywhere or MAP_FIXED
//! over its own maps; and stores a byte through a writable map, which on
//! Linux is the file's byte at once, and no byte of it past its end.

use super::super::random::Regs;
use super::model::{ix, World, PATHS};
use super::support::{map, may, writer, Entry};

pub fn map_one(w: &mut World, r: &mut Regs, pid: u32, i: usize) {
    let e = map(pid, r.small(24), 1 + r.small(4), "/f", r.small(6));
    let e = Entry { path: PATHS[i].to_vec(), ..e };
    let e = match r.small(3) {
        0 => e,
        1 => may(e),
        _ => writer(e),
    };
    let meets = |o: &Entry| o.pid == pid && o.at < e.place().1 && e.at < o.place().1;
    let over_own = w.table.iter().any(meets);
    /* Without MAP_FIXED the capsule would have placed it elsewhere. */
    if !over_own || r.small(2) == 0 {
        let _ = w.map(e, over_own);
    }
}

pub fn store(w: &mut World, r: &mut Regs) {
    let mine: Vec<Entry> = w.table.iter().filter(|e| e.writer && !e.orphan).cloned().collect();
    let Some(e) = mine.get(r.small(mine.len() as u64 + 1) as usize) else {
        return;
    };
    let (b, byte) = (e.off + r.small(e.span), r.small(256) as u8);
    w.mem[e.pid as usize][e.addr(b) as usize] = byte;
    /* Past the end a store is no byte of the file's: Linux drops it. */
    if let Some(t) = w.truth[ix(&e.path)].get_mut(b as usize) {
        *t = byte;
    }
}
