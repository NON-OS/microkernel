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

use super::active::Active;

/// The byte path: the same three calls on every UART here.
impl Active {
    pub(crate) fn tx_ready(&self) -> bool {
        match self {
            Self::Pl011(u) => u.tx_ready(),
            Self::Ns16550(u) | Self::MiniUart(u) => u.tx_ready(),
        }
    }

    pub(crate) fn send(&self, byte: u8) {
        match self {
            Self::Pl011(u) => u.send(byte),
            Self::Ns16550(u) | Self::MiniUart(u) => u.send(byte),
        }
    }

    pub(crate) fn receive(&self) -> Option<u8> {
        match self {
            Self::Pl011(u) => u.receive(),
            Self::Ns16550(u) | Self::MiniUart(u) => u.receive(),
        }
    }
}
