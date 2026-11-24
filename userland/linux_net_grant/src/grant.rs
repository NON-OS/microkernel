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

//! What a family was granted on.

use alloc::vec::Vec;

/// One family's grant, by what it rests on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Grant {
    /// Its program's own policy grants NET_ANON: any destination over the
    /// anonymity network, as the policy says.
    Policy { program: [u8; 32] },
    /// The run's box names destinations: those alone, as the canonical list
    /// the personality holds its own streams to (nonos_linux_policy::dests).
    Boxed { dests: Vec<u8> },
}
