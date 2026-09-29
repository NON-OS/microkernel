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

use crate::browser::dom::node::Node;

use super::super::computed::Computed;
use super::PseudoText;

/* Whether pseudo-element `kind` exists on element `node`, styled `host`:
 * ::before and ::after when a rule gives them `content`; ::marker on a
 * list item; ::placeholder on a field showing a placeholder;
 * ::file-selector-button on a file input; ::first-letter and ::first-line
 * on a block container. A universal ::marker or ::placeholder rule then
 * costs nothing on the other elements. */
pub(super) fn exists(kind: u8, node: &Node, host: &Computed, content: bool) -> bool {
    let tag = node.tag.as_str();
    let file = |t: &str| t.eq_ignore_ascii_case("file");
    match kind {
        PseudoText::BEFORE | PseudoText::AFTER => content,
        PseudoText::MARKER => matches!(tag, "li" | "summary"),
        PseudoText::PLACEHOLDER => {
            matches!(tag, "input" | "textarea") && node.attr("placeholder").is_some()
        }
        PseudoText::FILE_BUTTON => tag == "input" && node.attr("type").is_some_and(file),
        PseudoText::FIRST_LETTER | PseudoText::FIRST_LINE => host.is_block || host.is_inline_block,
        _ => false,
    }
}
