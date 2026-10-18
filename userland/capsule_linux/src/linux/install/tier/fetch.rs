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

//! A shipped tier whose models the volume lacks, handed to the model
//! fetcher, and every file held to its pin again after.

use alloc::format;

use super::super::apps::{App, STICK_TIER};
use super::super::fetch::fetcher::fetch;
use super::super::model_dep::{not_started, settle};
use super::super::why::Why;
use super::install::{models, refused, say};

pub(super) fn fetch_tier(app: &App, lacking: usize) -> Result<(), Why> {
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
    let direct = crate::linux::request_install::direct_asked();
    let status = fetch(app.tier(), direct).map_err(|rc| {
        let e = rc.saturating_neg();
        say(&format!("[LINUX] {}: the model fetcher did not start, errno {e}\n", app.name));
        not_started(rc)
    })?;
    say(&format!("[LINUX] {}: the model fetcher ended, status {status}\n", app.name));
    settle(status, &models(app)).inspect_err(|why| refused(app, *why))?;
    Ok(())
}
