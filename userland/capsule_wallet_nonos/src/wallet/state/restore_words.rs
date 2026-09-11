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

//! What the status line says about a vault this machine will not open.
//!
//! Two different situations and two different next steps, so they are not one
//! sentence. A machine that changed can be changed back, by booting the
//! firmware and kernel the vault was sealed under. A vault from another
//! machine never opens here, and the phrase is the only way in.

pub(super) fn sealed(machine_changed: bool) -> &'static [u8] {
    if machine_changed {
        b"a wallet is stored here, but this machine has changed since"
    } else {
        b"a wallet is stored here, sealed by another machine"
    }
}
