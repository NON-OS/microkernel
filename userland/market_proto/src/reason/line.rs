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

//! A reason as a client shows it: one sentence and whether to offer Retry.

use super::*;

/// What an installed listing is, said honestly. A Linux package is held in
/// memory until restart on every boot (the store is not kept at rest). A
/// Qwen tier's model is kept on an installed system's data volume, and on
/// a live session that volume is in memory, gone at power off.
pub fn installed_line(listing: &[u8]) -> &'static str {
    match listing.starts_with(b"linux.qwen-") {
        true => {
            "Installed. Enter opens it. On a live session its model is in memory, gone at power off"
        }
        false => "Installed for this session, held in memory until restart. Enter opens it",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reason {
    /// One sentence for a person.
    pub line: &'static str,
    /// Whether asking again could succeed without the machine changing.
    pub retry: bool,
}

pub(super) const fn said(line: &'static str, retry: bool) -> Reason {
    Reason { line, retry }
}

/// The reason for a code this system does not name.
const UNNAMED: Reason = said(
    "The install stopped for a reason this system does not name; retry, and if it \
     stops again the serial log says why",
    true,
);

/// One reason for every code: its group's, or UNNAMED.
pub fn reason(code: u8) -> Reason {
    [package_reason, removal_reason, volume_reason, model_reason, network_reason, exits_reason]
        .iter()
        .find_map(|of| of(code))
        .unwrap_or(UNNAMED)
}
