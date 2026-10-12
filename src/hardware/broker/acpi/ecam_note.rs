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

//! Whether every MCFG ECAM window lies inside the motherboard resource
//! ranges firmware declares, carried on the posture line once it is known
//! not to. The posture line is printed when the verdict changes, so the
//! finding is said once and then kept where the boot matrix reads it.

use core::sync::atomic::{AtomicBool, Ordering};

static ECAM_UNRESERVED: AtomicBool = AtomicBool::new(false);

pub(super) fn ecam_reserved(reserved: bool) {
    if ECAM_UNRESERVED.swap(!reserved, Ordering::Relaxed) == reserved {
        super::report::posture();
    }
}

pub(super) fn suffix() -> &'static [u8] {
    if ECAM_UNRESERVED.load(Ordering::Relaxed) {
        b" ECAM not reserved"
    } else {
        b""
    }
}
