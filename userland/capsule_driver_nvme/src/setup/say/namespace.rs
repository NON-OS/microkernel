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

//! The lines for which namespace was found and the geometry it is served with.

use crate::admin::NamespaceIdentity;
use crate::log::{emit, Line};
use crate::nvm::NamespaceGeometry;

pub fn no_namespace(namespace_count: u32) {
    let mut line = Line::new();
    line.text(b"no active namespace (NN ")
        .dec(namespace_count as u64)
        .text(b"); identify and health only");
    emit(&mut line);
}

pub fn list_refused() {
    let mut line = Line::new();
    line.text(b"active namespace list refused; trying NSID 1");
    emit(&mut line);
}

pub fn geometry(ns: &NamespaceIdentity, g: &NamespaceGeometry) {
    let mut line = Line::new();
    line.text(b"nsid ")
        .dec(ns.nsid as u64)
        .text(b": ")
        .dec(g.capacity_sectors)
        .text(b" LBAs of ")
        .dec(g.lba_size as u64)
        .text(b" bytes, ")
        .dec(g.max_sectors as u64)
        .text(b" per command");
    emit(&mut line);
}
