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

//! What the record checker found, with what the header says about whose
//! record it is, and what could not be checked offline.

use super::Verdict;

/// Offline, the registry entries cannot be checked (nsh-v1-attest.md step
/// 7): a record does not carry them. Their root is still bound, because
/// the quote's `extraData` covers it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Registry {
    RootThroughQuote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Report {
    pub verdict: Verdict,
    pub session_id: [u8; 16],
    pub subject: [u8; 64],
    pub start_ms: u64,
    pub registry: Registry,
}
