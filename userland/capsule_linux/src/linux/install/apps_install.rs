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
 * listed, and the model is a dependency, never a package (`model_dep`).
 * Each model file the volume holds under its pin is kept as it is, and one
 * it lacks is imported from the disk plan when the disk carries it. When
 * any is still missing, the tier is handed to the model fetcher, and every
 * file is held to its pin again before the tier counts as installed. The
 * store shows the installer as installing all the while, and its exit
 * code, the reason, when it stops.
 */

use alloc::format;
use alloc::vec::Vec;

use nonos_libc::mk_debug;

use super::apps::App;
use super::fetcher::fetch;
use super::model_dep::{needs_fetch, not_started, settle};
use super::why::Why;
use super::apps::STICK_TIER;
use crate::linux::file::{key, store_hash, HashFault};

const MAX_PROGRAM: u64 = 64 << 20;

pub fn install(app: &App, pin: &[u8; 32]) -> Result<(), Why> {
    let path = core::str::from_utf8(app.program).unwrap_or("its program");
    let hash = match store_hash(&key(app.program), MAX_PROGRAM) {
        Ok(hash) => hash,
        Err(HashFault::Absent(why)) => {
            say(&format!("[LINUX] {}: its program {path} is not in the store ({why})\n", app.name));
            /* A store vfs never loaded on this boot has no program in it either. */
            if let Ok(code @ 1..) = nonos_app_skeleton::clients::vfs::store_status() {
                let store = crate::linux::store_why::store_why(code);
                say(&format!("[LINUX] the store did not load on this boot: {store} (status {code})\n"));
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
    let have = models(app);
    let missing = needs_fetch(&have).inspect_err(|why| refused(app, *why))?;
    let lacking = have.iter().filter(|h| h.is_err()).count();
    if missing {
        say(&format!(
            "[LINUX] {}: model tier {}, {lacking} of {} files not on the volume; \
             handing it to the model fetcher\n",
            app.name,
            app.tier(),
            app.models.len()
        ));
        /*
         * The stick tier was imported from the stick above when this boot's
         * disk carries it; it was not, so it is downloaded, and that is said.
         */
        if app.tier() == STICK_TIER {
            say(&format!(
                "[LINUX] {}: this boot's disk carries no copy of the stick tier to import, \
                 so it is downloaded\n",
                app.name
            ));
        }
        let direct = crate::linux::request::direct_asked();
        let status = fetch(app.tier(), direct).map_err(|rc| {
            let e = rc.saturating_neg();
            say(&format!("[LINUX] {}: the model fetcher did not start, errno {e}\n", app.name));
            not_started(rc)
        })?;
        say(&format!("[LINUX] {}: the model fetcher ended, status {status}\n", app.name));
        settle(status, &models(app)).inspect_err(|why| refused(app, *why))?;
    }
    say(&format!(
        "[LINUX] {} installed: {} model files verified against their pins, {lacking} fetched\n",
        app.name,
        app.models.len()
    ));
    Ok(())
}

/*
 * Each model file's answer: its length once the kernel's import record of
 * it is the pin, else the errno. A pinned file the volume lacks is imported
 * from the disk plan first when the disk carries it.
 */
fn models(app: &App) -> Vec<Result<u64, i64>> {
    app.models.iter().map(|m| crate::linux::file::models::ensure(m)).collect()
}

fn refused(app: &App, why: Why) {
    say(&format!("[LINUX] {}: its model is not installed, reason {}\n", app.name, why.code()));
}

fn say(line: &str) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
