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

use alloc::vec::Vec;

use crate::browser::dom::node::Node;

use super::decl::Decl;
use super::parse::parse_decls;

/* The declarations of an element's style attribute, which cascade after
 * every author rule of their importance. */
pub(super) fn style_attr(node: &Node) -> Vec<Decl> {
    node.attr("style").map(parse_decls).unwrap_or_default()
}
