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

use crate::calc::fixed::{Fixed, FRAC};
use crate::calc::state::ErrorKind;

/// n! for a whole, non-negative n. Only a negative or fractional n is "Not
/// defined"; a factorial past what the fixed point holds (28! is the largest)
/// is "Result too large", which is what the readout says for every other
/// overflow. The product overflows by 34!, so a huge n stops within a few
/// dozen steps rather than counting up to it.
pub fn factorial(value: Fixed) -> Result<Fixed, ErrorKind> {
    if value < 0 || value % FRAC != 0 {
        return Err(ErrorKind::DomainError);
    }
    let n = value / FRAC;
    let mut acc: Fixed = 1;
    let mut k: Fixed = 2;
    while k <= n {
        acc = acc.checked_mul(k).ok_or(ErrorKind::Overflow)?;
        k += 1;
    }
    acc.checked_mul(FRAC).ok_or(ErrorKind::Overflow)
}
