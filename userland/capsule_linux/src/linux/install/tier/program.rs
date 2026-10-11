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

//! A shipped tier's program, held to the hash the market listed for it
//! before any of its models is looked at.

use alloc::format;

use super::super::apps::App;
use super::super::why::Why;
use super::install::say;
use crate::linux::file::{key, store_hash, HashFault};

const MAX_PROGRAM: u64 = 64 << 20;

pub(super) fn pinned(app: &App, pin: &[u8; 32]) -> Result<(), Why> {
    let path = core::str::from_utf8(app.program).unwrap_or("its program");
    let hash = match store_hash(&key(app.program), MAX_PROGRAM) {
        Ok(hash) => hash,
        Err(HashFault::Absent(why)) => {
            say(&format!("[LINUX] {}: its program {path} is not in the store ({why})\n", app.name));
            /* A store vfs never loaded on this boot has no program in it either. */
            if let Ok(code @ 1..) = nonos_app_skeleton::clients::vfs::store_status() {
                let store = crate::linux::store_why::store_why(code);
                say(&format!(
                    "[LINUX] the store did not load on this boot: {store} (status {code})\n"
                ));
            }
            return Err(Why::ProgramMissing);
        }
        Err(HashFault::Unread(why)) => {
            say(&format!("[LINUX] {}: its program {path} could not be read: {why}\n", app.name));
            return Err(Why::ProgramUnreadable);
        }
    };
    if &hash != pin {
        say(&format!("[LINUX] {}: the program is not the one the market listed\n", app.name));
        return Err(Why::Package);
    }
    Ok(())
}
