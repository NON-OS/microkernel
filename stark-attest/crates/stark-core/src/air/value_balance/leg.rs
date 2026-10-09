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

use super::super::super::field::Fp;

/// Which side of the balance a row sits on. Batch layout is public structure, not
/// witness, so this rides a periodic column: a prover cannot choose which row is
/// an input.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Leg {
    Input,
    Output,
    Pad,
}

impl Leg {
    pub(super) fn sign(self) -> Fp {
        match self {
            Leg::Input => Fp::ONE,
            Leg::Output => Fp::ZERO - Fp::ONE,
            Leg::Pad => Fp::ZERO,
        }
    }
}
