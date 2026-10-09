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

//! One staging slice, and the end of staging.

use crate::blk::error::BlkError;
use crate::blk::load::{Load, Step};
use crate::blk::patience::{gives_up, not_yet};
use crate::store::Store;

use super::seeder::{note, PackageSeeder, SLICE_MS};

impl PackageSeeder {
    /// One staging slice, from whichever path found it due.
    pub(super) fn advance(&mut self, store: &mut Store) {
        self.last_slice_ms = nonos_libc::mk_uptime_ms();
        if self.load.is_none() {
            self.attempts += 1;
            match Load::begin() {
                Ok(load) => self.load = Some(load),
                Err(e) => return self.failed(&e),
            }
        }
        let Some(load) = self.load.as_mut() else {
            return;
        };
        match load.step_for(SLICE_MS) {
            Step::More => {}
            Step::Done(staged, refused) => {
                store.adopt_staged(staged);
                crate::blk::status::loaded(refused);
                self.load = None;
                if refused > 0 {
                    self.finish(b"[VFSD] packages staged, damaged entries left out\n");
                } else {
                    self.finish(b"[VFSD] packages staged\n");
                }
            }
            Step::Failed(e) => {
                self.load = None;
                self.failed(&e);
            }
        }
    }

    /// An attempt that failed with `e`: tried again, said once while a disk
    /// not there yet is waited for, or given up on with why.
    fn failed(&mut self, e: &BlkError) {
        crate::blk::status::record(e);
        let waited = nonos_libc::mk_uptime_ms().saturating_sub(self.started_ms);
        // A disk not there yet is never given up on: a USB stick can bind long
        // after the wait, once its port is reset and its medium spins up. Past
        // the wait the desktop is told how it stands (settle), and the store is
        // still asked for, so a late stick loads its packages all the same.
        if gives_up(e, self.attempts, waited) && not_yet(e) {
            if !crate::blk::status::settled() {
                crate::blk::status::settle();
                let line = alloc::format!(
                    "[VFSD] no disk with the store after {} ms ({:?}); still looking\n",
                    waited,
                    e
                );
                note(line.as_bytes());
            }
            return;
        }
        if gives_up(e, self.attempts, waited) {
            let line = alloc::format!(
                "[VFSD] packages unavailable after {} attempts in {} ms: {:?}\n",
                self.attempts,
                waited,
                e
            );
            return self.finish(line.as_bytes());
        }
        if self.said != Some(*e) {
            self.said = Some(*e);
            let why = if not_yet(e) { "no disk with the store answers yet" } else { "the store did not load" };
            let line = alloc::format!(
                "[VFSD] store read, attempt {} at {} ms: {:?}; {}, asked again\n",
                self.attempts,
                waited,
                e,
                why
            );
            note(line.as_bytes());
        }
    }

    /// However staging ends, a reader asking whether a missing file is missing
    /// or not yet loaded gets a definite answer from here on.
    fn finish(&mut self, line: &[u8]) {
        self.done = true;
        crate::blk::status::settle();
        note(line);
    }
}
