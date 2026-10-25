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

//! Which controllers to try first, at what speed, and which declared
//! devices are within reach.

use crate::discover::{AcpiTouchpad, Found, MAX_CONTROLLERS};
use crate::transaction::Reach;

/// The declared addresses a client may reach on `dev` while no device is
/// bound: those of the devices that may sit on it (pairing.rs in discover),
/// never one whose record names another controller.
pub fn declared(dev: &Found, targets: &[AcpiTouchpad]) -> Reach {
    let here = targets.iter().filter(|tg| tg.addressed() && tg.may_be_on(dev));
    Reach::declared(here.map(|tg| (tg.addr, tg.desc_reg)))
}

/// Controllers in probe order: those a declared device names first (in
/// declaration order), then the rest in discovery order.
pub fn probe_order(cands: &[Found], targets: &[AcpiTouchpad]) -> ([usize; MAX_CONTROLLERS], usize) {
    let mut out = [0usize; MAX_CONTROLLERS];
    let mut n = 0usize;
    let named = |i: usize| targets.iter().any(|tg| tg.names(&cands[i]));
    for pass in [true, false] {
        for i in 0..cands.len().min(MAX_CONTROLLERS) {
            if named(i) == pass && !out[..n].contains(&i) {
                out[n] = i;
                n += 1;
            }
        }
    }
    (out, n)
}

/// Standard mode when a device this controller may carry declared a speed
/// below 400 kHz: one it is named by, or one that names no controller.
pub fn standard_mode(dev: &Found, targets: &[AcpiTouchpad]) -> bool {
    targets.iter().any(|tg| tg.may_be_on(dev) && tg.wants_standard_mode())
}
