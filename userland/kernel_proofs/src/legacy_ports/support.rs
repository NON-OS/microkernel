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

use super::compat::{ide, Ports, IDE_CLASS, IDE_SUBCLASS};
use super::dispi::DISPI;
use super::overlap;

pub(super) const fn p(base: u16, count: u16) -> Ports {
    Ports { base, count }
}

pub(super) fn meets(a: Ports, b: Ports) -> bool {
    overlap::meets((a.base, a.count), (b.base, b.count))
}

/// Every fixed window the broker can hand out.
pub(super) fn fixed() -> Vec<Ports> {
    let f = ide(IDE_CLASS, IDE_SUBCLASS, 0x00);
    f.bars.iter().flatten().copied().chain([DISPI]).collect()
}
