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

use crate::menu::{MenuAction, SecurityMode};

// Hardened is index 0 and Standard index 1; run::default_index picks the
// timeout selection by build policy. Development is intentionally absent: an unsigned,
// unattested boot is only reachable through the explicit dev override,
// never from this menu.
/*
 * Install NONOS sits after Standard so the default index does not move. It
 * boots this same signed kernel through the same checks, the way a recovery
 * or setup entry on an installer disk does.
 */
pub(super) const ENTRIES: [MenuAction; 7] = [
    MenuAction::Boot(SecurityMode::Hardened),
    MenuAction::Boot(SecurityMode::Standard),
    MenuAction::Install,
    MenuAction::SafeMode,
    MenuAction::NetworkIsolated,
    MenuAction::Recovery,
    MenuAction::Shutdown,
];
