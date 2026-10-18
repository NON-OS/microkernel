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

//! Error type the CLI surfaces. Variants map to distinct exit
//! codes so a caller (build script, CI job, deploy script) can
//! tell a malformed input from a key-handling failure from a
//! signature verdict.

mod display;
mod exit_code;

#[derive(Debug)]
pub enum CliError {
    /// I/O failure reading or writing a file.
    Io(std::io::Error),
    /// JSON input failed to parse or did not match the schema.
    Json(String),
    /// Hex-decoded field had the wrong length.
    BadHexLen { field: &'static str, expected: usize, got: usize },
    /// Unknown enum string in JSON input.
    UnknownEnum { field: &'static str, value: String },
    /// Operator key file or env var missing or malformed.
    KeyError(String),
    /// Binary blob did not decode against the marketplace_abi codec.
    DecodeFailed(String),
    /// Signature did not verify against the supplied pubkey.
    SignatureRefused,
    /// Embedded operator pubkey did not match the externally-
    /// supplied trust pubkey.
    OperatorPubkeyMismatch,
    /// Listing publisher pubkey did not match the signing key.
    PublisherPubkeyMismatch,
    /// Index serial is at or below the previous-serial argument.
    SerialRollback { previous: u64, current: u64 },
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
    }
}
