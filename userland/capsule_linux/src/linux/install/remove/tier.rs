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

//! A shipped Qwen tier's uninstall: its program is the image's and stays.

use alloc::format;

use nonos_libc::mk_debug;

use super::super::model_remove::{remove_not_started, removed};
use super::super::Why;

/// A shipped tier: its program is the image's and stays; its model files go
/// off the data volume by the model fetcher's `remove` (`model_remove`).
pub(super) fn tier(pkg: &str, tier: &str) -> Result<(), Why> {
    let status = super::super::fetch::fetcher::remove(tier).map_err(|rc| {
        say(&format!("[LINUX] {pkg}: the model fetcher did not start, errno {}\n", -rc));
        remove_not_started(rc)
    })?;
    say(&format!("[LINUX] {pkg}: the model fetcher removed its model, status {status}\n"));
    removed(status)
}

pub(super) fn say(line: &str) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
