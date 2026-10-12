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

//! Why a step of pairing was refused.

use nonos_bt_wire::WireError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmpError {
    /// A PDU was malformed.
    Wire(WireError),
    /// The peer's public key is not a point on P-256, or is the point at
    /// infinity (Vol 3, Part H, 2.3.5.6.1).
    BadKey,
    /// The peer sent the specification's published debug key, whose private
    /// key anyone can read (Vol 3, Part H, 2.3.5.6.1).
    DebugKey,
    /// Secret material that is zero or not below the curve order.
    BadSecret,
    /// A field holds a value this host does not accept here.
    Refused,
}

impl From<WireError> for SmpError {
    fn from(e: WireError) -> Self {
        Self::Wire(e)
    }
}
