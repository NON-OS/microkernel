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

//! Whether this install is of a market release, whose package's programs are
//! held (minted.rs) until the release's policy is bound to them. The install
//! role installs one package a process, so the word is set once, before the
//! package is fetched.

use core::sync::atomic::{AtomicBool, Ordering};

static HELD: AtomicBool = AtomicBool::new(false);

pub(in crate::linux::install) fn hold() {
    HELD.store(true, Ordering::SeqCst);
}

pub(in crate::linux::install) fn held() -> bool {
    HELD.load(Ordering::SeqCst)
}
