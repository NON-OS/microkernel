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

//! _TSD (ACPI 6.5, 8.4.6; recalled, unverified): which processors share a
//! throttling domain, and how a state is set across it.

use super::tss::words;
use crate::object::Object;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Domain {
    pub domain: u32,
    /// 0xFC all processors in software, 0xFD any one, 0xFE hardware.
    pub coordination: u32,
    pub processors: u32,
}

/// _TSD's one entry: five integers, the first saying there are five.
pub fn parse_tsd(o: &Object) -> Option<Domain> {
    let Object::Package(p) = o else { return None };
    let [count, _revision, domain, coordination, processors] = words::<5>(p.borrow().first()?)?;
    (count == 5).then_some(Domain { domain, coordination, processors })
}
