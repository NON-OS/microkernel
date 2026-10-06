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

//! Whether the legacy line is worth asking for once MSI-X and MSI were
//! refused. Only when the function has an INTx pin and firmware routed it:
//! firmware that expects MSI leaves the line at 0xFF, and such a controller is
//! still driven by polling. Pure, so the rule is held on the host
//! (hda_proofs).

/// The interrupt line firmware leaves when it routed none.
pub const NO_LINE: u8 = 0xFF;

/// Whether the legacy INTx line is worth asking the broker for.
pub const fn intx_routed(pin: u8, line: u8) -> bool {
    pin != 0 && line != NO_LINE
}
