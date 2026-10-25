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

//! bootfb's exit, and the exit orphaning G1's claim, in the takeover model
//! (L79). bootfb's threads are all off a CPU before its exit begins, so it
//! begins only between their calls and they take no step after; the exit
//! moves its claim on by `exited`, drains its grants and leaves the pages,
//! which nobody runs on now, then settles and its tables are freed.

use super::events::EXITED;
use super::give_back::RETIRE;
use super::holder::{ENTER, LOOKUP};
use super::holder_map::STORE;
use super::stage_rule::exited;
use super::types::Stage;
use super::world::{code, stage, END};
use super::world_state::World;

pub(super) fn exit(w: &mut World) -> bool {
    match w.pc[4] {
        0 => {
            let between = matches!(w.pc[2], ENTER | LOOKUP | STORE | RETIRE | END);
            if !between || !matches!(w.pc[3], 0 | END) {
                return false;
            }
            (w.dead, w.pc[2], w.pc[3]) = (true, END, END);
            if let Some(e) = w.fb {
                let (s, counted) = exited(stage(e.stage));
                w.exit = counted.then_some(e.epoch);
                let more = u8::from(counted);
                w.edit(e.epoch, |e| (e.stage, e.leavers) = (code(s), e.leavers + more));
            }
            w.pc[4] = 1;
        }
        // release_all_for_pid, not in bootfb's own tables: no unmap.
        1 => (w.grant, w.pc[4]) = ([false; 2], 2),
        _ => {
            if w.exit.is_some_and(|epoch| w.settle(epoch)) {
                w.events |= EXITED;
            }
            (w.freed, w.pc[4]) = (true, END);
        }
    }
    true
}

/// G1's holder exits while its claim is being made: exit_begin orphans it,
/// and only the claiming call takes it back.
pub(super) fn orphan(w: &mut World) -> bool {
    let Some(s) = w.gpu[0].map(stage).filter(|&s| s == Stage::Making) else {
        return false;
    };
    w.gpu[0] = Some(code(exited(s).0));
    w.pc[6] = END;
    true
}
