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

//! The directory's and the hops' lines from a transport's report.

use super::writer::Writer;
use nonos_route_proof::RouteReport;

/// The line under a transport's row: directory, nodes and validity.
pub fn directory<'a>(r: &RouteReport, out: &'a mut [u8; 48]) -> &'a [u8] {
    let mut w = Writer { out, len: 0 };
    w.num(u64::from(r.signatures_verified));
    w.text(b" of ");
    w.num(u64::from(r.signatures_required));
    w.text(b" signatures, ");
    w.num(u64::from(r.nodes));
    w.text(b" nodes");
    w.done()
}

/// The hop line: "3 of 3 hops authenticated".
pub fn hops<'a>(r: &RouteReport, out: &'a mut [u8; 48]) -> &'a [u8] {
    let mut w = Writer { out, len: 0 };
    w.num(u64::from(r.hops_authenticated));
    w.text(b" of ");
    w.num(u64::from(r.hops));
    w.text(b" hops authenticated");
    w.done()
}
