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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PsciMethod {
    Hvc,
    Smc,
}

impl PsciMethod {
    /// Zero is left for "no conduit named", the state before the tree is read.
    pub(super) const fn as_u8(self) -> u8 {
        match self {
            Self::Smc => 1,
            Self::Hvc => 2,
        }
    }

    pub(super) const fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Smc),
            2 => Some(Self::Hvc),
            _ => None,
        }
    }
}

pub fn set_method(method: PsciMethod) {
    super::state::set_method(method);
}

/// Whether the device tree named a PSCI conduit.
pub fn is_present() -> bool {
    super::state::method().is_some()
}
