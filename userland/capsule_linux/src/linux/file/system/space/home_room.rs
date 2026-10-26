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
 * The family's home held to its quota (declared::HOME, HOME_NAMES) and
 * every home to theirs together (declared::HOMES), and the person told,
 * once a run, when it is full.
 */

use core::sync::atomic::{AtomicBool, Ordering};

use super::super::super::private;
use super::quota::{home_allows, Kept};
use super::used::{copies, under};

static TOLD: AtomicBool = AtomicBool::new(false);

const FULL: &[u8] = b"linux: this app's home is full (16 MiB, 128 names, 64 MiB for every home). \
    Remove files from it, or uninstall the app to erase it.\n";

/* `whole` as used::room: `more` is in the copies already, or is added. */
pub(super) fn room(home: &[u8], more: Kept, whole: bool) -> Result<(), i64> {
    let now = copies(under(home)?);
    let all = copies(under(private::HOMES)?).bytes;
    let refused = match whole {
        true => home_allows(more, now, (0, all)),
        false => home_allows(now, now.plus(more), (all, all.saturating_add(more.bytes))),
    };
    if refused.is_err() && !TOLD.swap(true, Ordering::Relaxed) {
        crate::linux::say::say(FULL);
    }
    refused
}
