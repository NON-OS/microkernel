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

//! The firmware's TCG event log, as the loader carried it, for the
//! document. A verifier replays it to the quoted PCRs 0, 1, 2, 4 and 7.

use alloc::vec::Vec;

use super::document::log_fits;
use super::error::AttestDocError;
use crate::boot::handoff::types::MODULE_KIND_TCG_LOG;
use crate::security::boot::modules::module_bytes;

/// The most the loader's log module is looked at, so a longer log is told
/// apart from none.
const LOOK: u64 = 4 * 1024 * 1024;

/// The log, or empty when the loader carried none: a document still
/// answers, and a verifier that needs the log refuses it. A log over
/// `MAX_LOG` is refused, never cut.
pub(super) fn tcg_log() -> Result<Vec<u8>, AttestDocError> {
    match module_bytes(MODULE_KIND_TCG_LOG, LOOK) {
        None => Ok(Vec::new()),
        Some(log) if !log_fits(log) => Err(AttestDocError::LogTooLarge),
        Some(log) => Ok(Vec::from(log)),
    }
}
