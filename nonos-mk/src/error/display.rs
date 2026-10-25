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

//! The message each CliError prints.

use std::fmt;

use super::CliError;

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Io(e) => write!(f, "io: {e}"),
            CliError::Json(s) => write!(f, "json: {s}"),
            CliError::BadHexLen { field, expected, got } => {
                write!(f, "{field}: expected {expected} hex bytes, got {got}")
            }
            CliError::UnknownEnum { field, value } => {
                write!(f, "{field}: unknown value {value:?}")
            }
            CliError::KeyError(s) => write!(f, "key: {s}"),
            CliError::DecodeFailed(s) => write!(f, "decode: {s}"),
            CliError::SignatureRefused => write!(f, "signature did not verify against pubkey"),
            CliError::OperatorPubkeyMismatch => {
                write!(f, "embedded operator_pubkey does not match supplied pubkey")
            }
            CliError::PublisherPubkeyMismatch => {
                write!(f, "listing publisher_pubkey does not match signing key")
            }
            CliError::SerialRollback { previous, current } => {
                write!(f, "serial rollback: previous={previous} current={current}")
            }
        }
    }
}
