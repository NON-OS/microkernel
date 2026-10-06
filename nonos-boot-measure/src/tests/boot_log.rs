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

use super::log_build::{digest, ev, extend, standard, EV_APP, EV_EFI_ACTION, EV_SEPARATOR};

pub(super) const LOADER: u8 = 1;

/// A boot as OVMF logs it: firmware in PCR 0, the boot attempt and the
/// separator in PCR 4, then the loader, then PCR 7 and the loader's own PCR 9.
pub(super) fn boot(after: &[Vec<u8>]) -> (Vec<u8>, [u8; 32]) {
    let mut log = standard();
    let mut pcr = [0u8; 32];
    for (p, k, d) in [
        (0, 1, 9),
        (4, EV_EFI_ACTION, 2),
        (4, EV_SEPARATOR, 3),
        (4, EV_APP, LOADER),
        (7, 6, 4),
        (9, 1, 5),
    ] {
        log.extend(ev(p, k, digest(d)));
        if p == 4 {
            pcr = extend(pcr, digest(d));
        }
    }
    for e in after {
        log.extend(e);
    }
    (log, pcr)
}
