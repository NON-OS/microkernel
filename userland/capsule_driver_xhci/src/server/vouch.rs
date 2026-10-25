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

//! A request the multiplexer has admitted on a slot the sender did not
//! enable: a claimant of a shared device (slots/gate.rs), or the class
//! driver of a child parked below a hub (mux_virt.rs). A controller serves
//! a slot only to the client that enabled it (`dispatch`), save one the
//! multiplexer vouched for, for this request alone: `set_sender` clears it.

use core::sync::atomic::Ordering;

use super::served::served;

pub fn vouch() {
    served(|s| s.vouched.store(true, Ordering::Relaxed));
}

pub(super) fn unvouch() {
    served(|s| s.vouched.store(false, Ordering::Relaxed));
}

pub fn vouched() -> bool {
    served(|s| s.vouched.load(Ordering::Relaxed))
}
