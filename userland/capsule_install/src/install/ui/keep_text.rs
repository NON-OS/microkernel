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

//! The confirm screen's paragraph on a disk whose data is not ready to
//! keep, word-wrapped where it is drawn. It is kept out of `screens/`,
//! whose lines the installer's layout proofs measure as single lines; they
//! hold this one, wrapped, to the confirm screen's body instead
//! (`userland/setup_layout_proofs/src/installer_keep_tests.rs`).

pub const NOT_READY: &str = "To keep your data, start your installed NØNOS, open Install NØNOS, choose Prepare this computer for an update, then start from this stick again.";
