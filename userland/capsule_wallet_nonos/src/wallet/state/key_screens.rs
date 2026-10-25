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
//! Which key screen is open, and the presses and toggles on it. What the
//! screens hold is in `custody`.

#[derive(Default)]
pub struct KeyScreens {
    /// The first press of a replace was made and said; the next goes on.
    pub armed: bool,
    /// The recovery words being typed are drawn, not masked.
    pub recover_shown: bool,
    /// Whether the import, the one-time backup, the restore and the export
    /// screens are open.
    pub import_active: bool,
    pub backup_active: bool,
    pub recover_active: bool,
    pub export_active: bool,
}
