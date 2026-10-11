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

use super::kind::UartKind;

/// The UARTs we can drive, by the `compatible` strings that name them.
pub(super) const NEEDLES: &[&[u8]] =
    &[b"arm,pl011", b"ns16550a", b"ns16550", b"brcm,bcm2835-aux-uart"];

/// What each needle names, index for index.
const KINDS: [UartKind; 4] =
    [UartKind::Pl011, UartKind::Ns16550, UartKind::Ns16550, UartKind::MiniUart];

pub(super) fn kind(needle: usize) -> UartKind {
    KINDS[needle]
}
