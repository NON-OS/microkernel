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

/// Marketplace operator, v1. Read from the key file so a scratch build can
/// stand in its own operator the way it stands in its own trust anchor. A
/// file that is not exactly 32 bytes does not compile.
pub(super) const NOX_OPERATOR_V1: [u8; 32] =
    *include_bytes!("../../../../.keys/marketplace_operator_ed25519.pub");

pub(super) const TRUSTED_OPERATORS: &[[u8; 32]] = &[NOX_OPERATOR_V1];
