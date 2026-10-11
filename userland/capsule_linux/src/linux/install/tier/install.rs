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

use super::super::apps::App;
use super::super::model_dep::needs_fetch;
use super::super::why::Why;

pub fn install(app: &App, pin: &[u8; 32]) -> Result<(), Why> {
    super::program::pinned(app, pin)?;
    let have = models(app);
    let missing = needs_fetch(&have).inspect_err(|why| refused(app, *why))?;
    let lacking = have.iter().filter(|h| h.is_err()).count();
    if missing {
        super::fetch::fetch_tier(app, lacking)?;
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
pub(super) fn models(app: &App) -> Vec<Result<u64, i64>> {
    app.models.iter().map(|m| crate::linux::file::models::ensure(m)).collect()
}

pub(super) fn refused(app: &App, why: Why) {
    say(&format!("[LINUX] {}: its model is not installed, reason {}\n", app.name, why.code()));
}

pub(super) fn say(line: &str) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
