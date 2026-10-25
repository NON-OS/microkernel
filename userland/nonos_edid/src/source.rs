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

/// Where in the EDID a timing was named.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// The established timings bitmaps, I and II or III.
    Established,
    /// A two byte standard timing, in the base block or a 0xFA descriptor.
    Standard,
    /// A three byte CVT code in a 0xF8 descriptor.
    Cvt,
    /// A detailed timing descriptor in the base block.
    Detailed,
    /// A CTA-861 short video descriptor, looked up in the VIC table.
    CtaVic,
    /// A detailed timing descriptor in a CTA-861 extension.
    CtaDetailed,
    /// A DisplayID timing: 1.3 types I to IV, 2.x types VII to IX.
    DisplayId,
}
