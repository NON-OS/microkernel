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
/*
 * Installing a shipped tier: the program must be the one the market
 * listed, and each model file it needs is imported from the disk plan and
 * kept only if it hashes to its pin. A tier whose model is not on the disk
 * is refused by name, and the store offers it again.
 */

use alloc::format;

use nonos_libc::mk_debug;

use super::apps::App;
use super::why::Why;
use crate::linux::file::{key, store_read};

const MAX_PROGRAM: u32 = 64 << 20;

pub fn install(app: &App, pin: &[u8; 32]) -> Result<(), Why> {
    let Ok(program) = store_read(&key(app.program), MAX_PROGRAM) else {
        say(&format!("[LINUX] {}: its program is not in the store\n", app.name));
        return Err(Why::NotProvided);
    };
    if blake3::hash(&program).as_bytes() != pin {
        say(&format!("[LINUX] {}: the program is not the one the market listed\n", app.name));
        return Err(Why::Package);
    }
    for model in app.models {
        if let Err(e) = crate::linux::file::models::ensure(model) {
            let name = core::str::from_utf8(model).unwrap_or("?");
            say(&format!("[LINUX] {}: model {name} not imported, errno {e}\n", app.name));
            return Err(Why::NotProvided);
        }
    }
    say(&format!("[LINUX] {} installed: {} model files verified\n", app.name, app.models.len()));
    Ok(())
}

fn say(line: &str) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
