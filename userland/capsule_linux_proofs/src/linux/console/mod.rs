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

//! The console's pure pieces a pseudo-terminal's discipline reuses, at the
//! crate paths it names in the capsule (`crate::linux::console`).

#[path = "../../../../capsule_linux/src/linux/console/termios.rs"]
pub mod termios;
// The rest are the crate's own console pieces, mounted once (console/mod.rs).
pub use crate::console::{queue, queue_piece, queue_take, tty_rules, wipe};
