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

//! Why a host was refused. Each refusal is one check of
//! docs/design/nsh-v1-attest.md, and the first one met ends the session
//! with `ABORT(POLICY)`. Its name, from `Refusal::name`, is what the user is
//! shown: `POLICY (pcr9)`.

/// How a document or a TPM structure failed to parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatFault {
    /// The bytes end inside a field.
    Short,
    /// Bytes are left after the last field.
    Trailing,
    /// A length is over its cap.
    Oversize,
    /// The magic is not `NONOSATT`.
    Magic,
    /// A field holds a value outside its range.
    Value,
}

/// How the quote's signed structure failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteFault {
    /// `TPMS_ATTEST` does not parse, or has bytes after `pcrDigest`.
    Malformed,
    /// It does not start with `TPM_GENERATED_VALUE`.
    NotTpmGenerated,
    /// Its type is not `TPM_ST_ATTEST_QUOTE`.
    NotAQuote,
}

#[path = "refusal_log.rs"]
mod log;
pub use log::LogFault;

/// What PCR 14's capsules fail: none is recorded, or one was admitted by
/// something other than its STARK proof under the vendor's root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsuleFault {
    None,
    PathOnly,
    Developer,
    LocalBuild,
}
