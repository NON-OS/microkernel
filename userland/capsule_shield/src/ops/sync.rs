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

//! Syncing with the pool, and what a scan's failure says.

use shield_wire::Values;

use super::held::wallet;
use super::Answer;

pub fn sync() -> Answer {
    let w = wallet()?;
    let s = w.sync_chain().map_err(scan_failed)?;
    let mut v = Values::new();
    v.put("head", &s.head.to_string())
        .put("received", &s.received.to_string())
        .put("deposited", &s.deposited.to_string())
        .put("spent", &s.spent.to_string())
        .put("wait_leaves", &s.wait_leaves.to_string())
        .put("wait_minutes", &s.wait_minutes.to_string());
    Ok(v)
}

/// A scan's failure, with the server it asked and the step it stopped at, so the wallet says
/// exactly where reading the pool broke: the chain head, the leaf count, a history, the
/// registry. Said on the serial console too.
pub(super) fn scan_failed(e: impl core::fmt::Display) -> String {
    let said = match nox_shield_core::net::rpc::progress::last_step() {
        Some((host, step)) => format!("{e}, while asking {host} for {step}"),
        None => e.to_string(),
    };
    crate::steps::serial(&format!("scan failed: {said}"));
    said
}
