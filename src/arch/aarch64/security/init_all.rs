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

use super::bti::init_bti;
use super::mte::init_mte;
use super::pan::init_pan;
use super::spectre::init_spectre_mitigations;

/// The protections each CPU turns on in its own SCTLR_EL1 and PSTATE, which
/// is why every CPU runs this, the boot CPU in `boot::init` and each
/// secondary in its entry. Each is gated on its own ID register field.
///
/// Pointer authentication is not here: its key has to go in from a function
/// that never returns (`security::pac`).
pub fn init_all() {
    init_pan();
    init_bti();
    init_mte();
    init_spectre_mitigations();
}
