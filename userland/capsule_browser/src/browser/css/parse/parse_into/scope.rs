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

use super::block::Parent;
use super::cx::Cx;
use super::parse_into;

/* @scope (root) { ... }: the block reads as rules nested in the scope
 * root, with :scope and & naming it. The '(limit)' bound after 'to' is
 * not modelled, so a scoped rule also reaches past its limit. */
pub(super) fn scope(cond: &str, body: &str, cx: &mut Cx, depth: u32, parent: Option<&Parent>) {
    let root = cond.trim().strip_prefix('(').and_then(|r| r.split(')').next());
    let root = root.map(str::trim).filter(|r| !r.is_empty()).unwrap_or(":root");
    if let Some(me) = Parent::new(root, parent) {
        parse_into(body, cx, depth + 1, Some(&me));
    }
}
