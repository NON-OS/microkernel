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

use core::sync::atomic::{AtomicU8, Ordering};

use super::method::PsciMethod;

/// No conduit until the device tree names one. Guessing SMC traps at EL1
/// with no EL3, and on a Pi 4 reaches an armstub that implements no PSCI.
static PSCI_METHOD: AtomicU8 = AtomicU8::new(0);

pub(super) fn set_method(method: PsciMethod) {
    PSCI_METHOD.store(method.as_u8(), Ordering::Release);
}

pub(super) fn method() -> Option<PsciMethod> {
    PsciMethod::from_u8(PSCI_METHOD.load(Ordering::Acquire))
}
