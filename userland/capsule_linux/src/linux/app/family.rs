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

//! What the family's tree shows, decided once by the program it was started
//! for. /home and /root are the family's own only when its policy grants
//! FILES_HOME; otherwise they are the shared tree's, read-only. /models is
//! there only when it grants MODELS_RO. A program exec'd later runs inside
//! the first one's box, so the first one's grant bounds the family.

use core::sync::atomic::{AtomicBool, Ordering};

use nonos_linux_policy::mask::{grants, FILES_HOME, MODELS_RO};

/// Nothing is granted until a program is proved into the family.
static HOME: AtomicBool = AtomicBool::new(false);
static MODELS: AtomicBool = AtomicBool::new(false);

/// The tree the family started for `mask` sees.
pub fn grant(mask: u64) {
    HOME.store(grants(mask, FILES_HOME), Ordering::Relaxed);
    MODELS.store(grants(mask, MODELS_RO), Ordering::Relaxed);
}

/// Whether /home and /root are the family's own.
pub fn home() -> bool {
    HOME.load(Ordering::Relaxed)
}

/// Whether /models is there at all.
pub fn models() -> bool {
    MODELS.load(Ordering::Relaxed)
}
