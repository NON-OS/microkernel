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

//! The exit code each CliError maps to, one per kind of failure.

use super::CliError;

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::Io(_) => 2,
            CliError::Json(_) | CliError::UnknownEnum { .. } | CliError::BadHexLen { .. } => 3,
            CliError::KeyError(_) => 4,
            CliError::DecodeFailed(_) => 5,
            CliError::SignatureRefused => 6,
            CliError::OperatorPubkeyMismatch | CliError::PublisherPubkeyMismatch => 7,
            CliError::SerialRollback { .. } => 8,
        }
    }
}
