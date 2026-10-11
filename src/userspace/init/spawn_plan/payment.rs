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

//! The meter (capsule_payment) comes up after the keyring and the vfs it
//! keeps its key and its channels with, and before anything can ask it for a
//! voucher.

#[cfg(feature = "nonos-capsule-payment")]
pub(super) fn spawn() {
    use crate::userspace::capsule_payment as c;
    super::boot::capsule("PAYMENT", "payment", c::spawn_payment_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-payment"))]
pub(super) fn spawn() {}
